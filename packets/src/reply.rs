use crate::request::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AuxillaryProperty {
    Nothing,
    Encrypted,
    Crc
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