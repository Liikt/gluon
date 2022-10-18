//! packets contains all the fun and cool packet types that are used to 
//! communicate with the client
//! 
//! TODO: implement CRC and encryption

use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientCommandType {
    Ack,
    Connect,
    PeerID,
    Disconnect,
    Unreliable,
    Fragmented,
    NotImplemented(u8)
}

impl Default for ClientCommandType {
    fn default() -> Self {
        ClientCommandType::NotImplemented(0xff)
    }
}

impl From<u8> for ClientCommandType {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::Ack,
            2 => Self::Connect,
            3 => Self::PeerID,
            4 => Self::Disconnect,
            7 => Self::Unreliable,
            8 => Self::Fragmented,
            _ => Self::NotImplemented(val)
        }
    }
}

impl Into<u8> for ClientCommandType {
    fn into(self) -> u8 {
        match self {
            Self::Ack => 1,
            Self::Connect => 2,
            Self::PeerID => 3,
            Self::Disconnect => 4,
            Self::Unreliable => 7,
            Self::Fragmented => 8,
            Self::NotImplemented(v) => v
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct CommandHeader {
    pub cmd_type: ClientCommandType,
    pub channel_id: u8,
    pub flags: u8,
    pub reserved: u8,
    pub size: u32,
    pub reliable_seq_num: u32,
    pub unreliable_seq_num: Option<u32>,
    pub start_seq_num: Option<u32>,
    pub fragment_count: Option<u32>,
    pub fragment_num: Option<u32>,
    pub total_len: Option<u32>,
    pub fragment_offset: Option<u32>
}

impl CommandHeader {
    pub fn new(
        cmd_type: ClientCommandType,
        channel_id: u8,
        flags: u8,
        reliable_seq_num: u32
    ) -> Self {
        assert_ne!(cmd_type, ClientCommandType::Unreliable);
        assert_ne!(cmd_type, ClientCommandType::Fragmented);

        Self {
            cmd_type,
            channel_id,
            flags,
            reserved: 4,
            size: 12,
            reliable_seq_num,
            ..Default::default()
        }
    }

    pub fn make_unreliable(mut self, seq_num: u32) -> Self {
        self.cmd_type = ClientCommandType::Unreliable;
        self.size = 0x10;
        self.unreliable_seq_num = Some(seq_num);
        self
    }

    pub fn make_fragmented(
        mut self,
        seq_num: u32,
        fragment_count: u32,
        fragment_num: u32,
        fragment_offset: u32,
        total_len: u32,
    ) -> Self {
        self.cmd_type = ClientCommandType::Fragmented;
        self.size = 0x20;
        self.start_seq_num = Some(seq_num);
        self.fragment_count = Some(fragment_count);
        self.fragment_num = Some(fragment_num);
        self.fragment_offset = Some(fragment_offset);
        self.total_len = Some(total_len);
        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ret = vec![0; self.len()];
        ret[0x0] = self.cmd_type.into();
        ret[0x1] = self.channel_id;
        ret[0x2] = self.flags;
        ret[0x3] = self.reserved;
        ret[0x4..0x8].copy_from_slice(&self.size.to_be_bytes());
        ret[0x8..0xc].copy_from_slice(&self.reliable_seq_num.to_be_bytes());

        match self.cmd_type {
            ClientCommandType::Unreliable => {
                ret[0xc..0x10].copy_from_slice(&self.unreliable_seq_num
                    .unwrap().to_be_bytes());
            }
            ClientCommandType::Fragmented => {
                ret[0xc..0x10].copy_from_slice(&self.start_seq_num.unwrap()
                    .to_be_bytes());
                ret[0x10..0x14].copy_from_slice(&self.fragment_count.unwrap()
                    .to_be_bytes());
                ret[0x14..0x18].copy_from_slice(&self.fragment_num.unwrap()
                    .to_be_bytes());
                ret[0x18..0x1c].copy_from_slice(&self.total_len.unwrap()
                    .to_be_bytes());
                ret[0x1c..0x20].copy_from_slice(&self.fragment_offset.unwrap()
                    .to_be_bytes());
            }
            _ => {}
        }

        ret
    }

    pub fn deserialize(buf: &[u8], mut offset: usize) -> Self {
        let mut ret = Self {
            cmd_type: ClientCommandType::from(buf[offset]),
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
            ClientCommandType::Unreliable => {
                ret.unreliable_seq_num = Some(u32::from_be_bytes(
                    buf[offset..offset+4].try_into().unwrap()));
            }
            ClientCommandType::Fragmented => {
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
            ClientCommandType::Unreliable => 0x10,
            ClientCommandType::Fragmented => 0x20,
            _ => 0xc
        }
    }
}

impl Debug for CommandHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "CmdHeader[ type: {:?}, channel_id: 0x{:x}, flags: 0x{:x}, reserved: 0x{:x}, size: 0x{:x}, reliable_sequence_num: 0x{:x}",
            self.cmd_type, self.channel_id, self.flags, self.reserved, self.size, self.reliable_seq_num
        )?;
        match self.cmd_type {
            ClientCommandType::Unreliable => write!(f, ", unreliable_sequence_num: 0x{:x}", 
                self.unreliable_seq_num.unwrap())?,
            ClientCommandType::Fragmented => write!(f, 
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
    pub header: CommandHeader,
    pub payload: Vec<u8>
}

impl Command {
    pub fn new(mut header: CommandHeader, payload: Vec<u8>) -> Self {
        header.size += payload.len() as u32;
        Self {
            header,
            payload
        }
    }

    pub fn deserialize(buf: &[u8], mut offset: usize) -> Self {
        let header = CommandHeader::deserialize(&buf, offset);
        offset += header.len() as usize;

        let payload_len = header.size as usize - header.len();
        let mut ret = Self {
            header,
            payload: vec![0; payload_len]
        };

        ret.payload.copy_from_slice(&buf[offset..offset+payload_len]);
        ret
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ret = vec![0; self.len() as usize];
        println!("{} {} {}", self.len(), self.header.len(), self.payload.len());
        ret[..self.header.len()].copy_from_slice(&self.header.serialize());
        ret[self.header.len()..].copy_from_slice(&self.payload);

        ret
    }

    fn len(&self) -> u32 {
        self.header.size
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.header.cmd_type {
            ClientCommandType::Ack => 
                write!(f, "Cmd[ {:?}, seq_num: 0x{:x}, sent_time: 0x{:x}]", 
                    self.header, u32::from_be_bytes(self.payload[0..4].try_into().unwrap()),
                    u32::from_be_bytes(self.payload[4..8].try_into().unwrap())),
            _ =>
                write!(f, "Cmd[ {:?}, payload: {:x?} ]", self.header, self.payload),
        }
    }
}

pub struct CommandPacket {
    pub peer_id: u16,
    pub use_crc: bool,
    pub cmd_count: u8,
    pub time: u32,
    pub challenge: u32,
    pub cmds: Vec<Command>
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
            let cmd= Command::deserialize(&val, offset);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AuxillaryProperty {
    Nothing = 0,
    Encrypted = 1,
    Crc = 204
}

impl Into<u8> for AuxillaryProperty {
    fn into(self) -> u8 {
        match self {
            Self::Nothing => 0,
            Self::Encrypted => 1,
            Self::Crc => 204
        }
    }
}

impl Default for AuxillaryProperty {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Debug, Clone, Default)]
pub struct Reply {
    aux_property: AuxillaryProperty,
    cmd_count: u8,
    send_time: u32,
    challenge: u32,
    cmds: Vec<Command>,

    _iv: Option<[u8; 16]>,
    _crc: Option<u32>
}

impl Reply {
    pub fn new(send_time: u32, challenge: u32, cmds: Vec<Command>) -> Self {
        Self {
            cmd_count: cmds.len().try_into().unwrap(),
            send_time,
            challenge,
            cmds,
            ..Default::default()
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let total_len = self.cmds.iter().map(|x| x.len()).sum::<u32>() + 12;
        let mut ret = vec![0; total_len as usize];
        ret[0x0..0x2].copy_from_slice(&[0x13, 0x37]);
        ret[0x2] = self.aux_property.into();
        ret[0x3] = self.cmd_count;
        ret[0x4..0x8].copy_from_slice(&self.send_time.to_be_bytes());
        ret[0x8..0xc].copy_from_slice(&self.challenge.to_be_bytes());
        let mut cur = 0xc;

        for cmd in &self.cmds {
            ret[cur..cur+cmd.len() as usize].copy_from_slice(&cmd.serialize());
            cur += cmd.len() as usize;
        }

        ret
    }
}