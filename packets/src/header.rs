use std::fmt::Debug;

use crate::typ::CommandType;

#[derive(Default, Clone, Copy)]
pub struct CommandHeader {
    pub cmd_type: CommandType,
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
        cmd_type: CommandType,
        channel_id: u8,
        flags: u8,
        reliable_seq_num: u32
    ) -> Self {
        assert_ne!(cmd_type, CommandType::Unreliable);
        assert_ne!(cmd_type, CommandType::Fragmented);

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
        self.cmd_type = CommandType::Unreliable;
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
        self.cmd_type = CommandType::Fragmented;
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
            CommandType::Unreliable => {
                ret[0xc..0x10].copy_from_slice(&self.unreliable_seq_num
                    .unwrap().to_be_bytes());
            }
            CommandType::Fragmented => {
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
            cmd_type: CommandType::from(buf[offset]),
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
            CommandType::Unreliable => {
                ret.unreliable_seq_num = Some(u32::from_be_bytes(
                    buf[offset..offset+4].try_into().unwrap()));
            }
            CommandType::Fragmented => {
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

    pub fn len(&self) -> usize {
        match self.cmd_type {
            CommandType::Unreliable => 0x10,
            CommandType::Fragmented => 0x20,
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
            CommandType::Unreliable => write!(f, ", unreliable_sequence_num: 0x{:x}", 
                self.unreliable_seq_num.unwrap())?,
            CommandType::Fragmented => write!(f, 
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
