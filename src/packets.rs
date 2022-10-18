use std::fmt::Debug;

#[derive(Default, Clone, Copy)]
pub struct CommandHeader {
    cmd_type: u8,
    channel_id: u8,
    flags: u8,
    reserved: u8,
    size: u32,
    reliable_seq_num: u32,
    unreliable_seq_num: Option<u32>,
    start_seq_num: Option<u32>,
    fragment_count: Option<u32>,
    fragment_num: Option<u32>,
    total_len: Option<u32>,
    fragment_offset: Option<u32>
}

impl CommandHeader {
    pub fn new(buf: &[u8], mut offset: usize) -> Self {
        let mut ret = Self {
            cmd_type: buf[offset],
            channel_id: buf[offset + 1],
            flags: buf[offset + 2],
            reserved: buf[offset + 3],
            size: u32::from_be_bytes(buf[offset+0x4..offset+0x8].try_into()
                .unwrap()),
            reliable_seq_num: u32::from_be_bytes(buf[offset+0x8..offset+0xc]
                .try_into().unwrap()),
            ..Default::default()
        };

        offset += 0xc;

        match ret.cmd_type {
            7 => {
                ret.unreliable_seq_num = Some(u32::from_be_bytes(
                    buf[offset..offset+4].try_into().unwrap()));
            }
            8 => {
                ret.start_seq_num = Some(u32::from_be_bytes(
                    buf[offset..offset+0x4].try_into().unwrap()));
                ret.fragment_count = Some(u32::from_be_bytes(
                    buf[offset+0x4..offset+0x8].try_into().unwrap()));
                ret.fragment_num = Some(u32::from_be_bytes(
                    buf[offset+0x8..offset+0xc].try_into().unwrap()));
                ret.total_len = Some(u32::from_be_bytes(
                    buf[offset+0xc..offset+0x10].try_into().unwrap()));
                ret.fragment_offset = Some(u32::from_be_bytes(
                    buf[offset+0x10..offset+0x14].try_into().unwrap()));
            },
            _ => {}
        }

        ret
    }

    fn len(&self) -> usize {
        match self.cmd_type {
            7 => 16,
            8 => 32,
            _ => 12
        }
    }
}

impl Debug for CommandHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "CmdHeader[ type: 0x{:x}, channel_id: 0x{:x}, flags: 0x{:x}, reserved: 0x{:x}, size: 0x{:x}, reliable_sequence_num: 0x{:x}",
            self.cmd_type, self.channel_id, self.flags, self.reserved, self.size, self.reliable_seq_num
        )?;
        match self.cmd_type {
            7 => write!(f, ", unreliable_sequence_num: 0x{:x}", 
                self.unreliable_seq_num.unwrap())?,
            8 => write!(f, 
                ", start_sequence_num: 0x{:x}, fragment_count: 0x{:x}, fragment_num: 0x{:x}, total_len: 0x{:x}, fragment_offset: 0x{:x}",
                self.start_seq_num.unwrap(), self.fragment_count.unwrap(), 
                self.fragment_num.unwrap(), self.total_len.unwrap(),
                self.fragment_offset.unwrap()
            )?,
            _ => {}
        }
        write!(f, " ]")
    }
}

#[derive(Clone)]
pub struct Command {
    header: CommandHeader,
    payload: Vec<u8>
}

impl Command {
    pub fn new(buf: &[u8], mut offset: usize) -> Self {
        let header = CommandHeader::new(&buf, offset);
        offset += header.len() as usize;

        let payload_len = header.size as usize - header.len();
        let mut ret = Self {
            header,
            payload: vec![0; payload_len]
        };

        ret.payload.copy_from_slice(&buf[offset..offset+payload_len]);
        ret
    }

    fn len(&self) -> u32 {
        self.header.size
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cmd[ {:?}, payload len: 0x{:x} ]", self.header, 
            self.payload.len())
    }
}

pub struct CommandPacket {
    peer_id: u16,
    use_crc: bool,
    cmd_count: u8,
    time: u32,
    challenge: u32,
    cmds: Vec<Command>
}

impl From<Vec<u8>> for CommandPacket {
    fn from(val: Vec<u8>) -> Self {
        let mut ret = Self { 
            peer_id: u16::from_be_bytes(val[0x0..0x2].try_into().unwrap()),
            use_crc: val[2] != 0,
            cmd_count: val[3],
            time: u32::from_be_bytes(val[0x4..0x8].try_into().unwrap()),
            challenge: u32::from_be_bytes(val[0x8..0xc].try_into().unwrap()),
            cmds: Vec::new()
        };

        let mut offset = 0xc;
        while offset < val.len() {
            let cmd= Command::new(&val, offset);
            offset += cmd.len() as usize;
            ret.cmds.push(cmd);
        }

        ret
    }
}

impl Debug for CommandPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet[ peer_id: 0x{:04x}, use_crc: {}, cmd_count: 0x{:x}, time: 0x{:08x}, challenge: 0x{:08x}, data: {:?}] ]", 
            self.peer_id, self.use_crc, self.cmd_count, self.time, 
            self.challenge, self.cmds)
    }
}