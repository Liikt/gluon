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

impl From<u8> for AuxillaryProperty {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Nothing,
            1 => Self::Encrypted,
            204 => Self::Crc,
            _ => panic!("{} is not an auxalliray property", val)
        }
    }
}

impl Default for AuxillaryProperty {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Clone, Default)]
pub struct Reply {
    _unused: u16,
    aux_property: AuxillaryProperty,
    cmd_count: u8,
    send_time: u32,
    challenge: u32,
    cmds: Vec<Command>,

    iv: Option<[u8; 16]>,
    crc: Option<u32>
}

impl std::fmt::Debug for Reply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reply[ aux_properties: {:?}, cmd_count: {}, send_time: 0x{:x}, challenge: 0x{:x}",
            self.aux_property, self.cmd_count, self.send_time, self.challenge)?;
        if let Some(iv) = self.iv {
            write!(f, ", iv: {:x?}", iv)?;
        }
        if let Some(crc) = self.crc {
            write!(f, ", crc: 0x{:x?}", crc)?;
        }
        write!(f, ", cmds: {:?} ]", self.cmds)
    }
}

impl Reply {
    pub fn _new(send_time: u32, challenge: u32, cmds: Vec<Command>) -> Self {
        Self {
            _unused: 0x1337,
            cmd_count: cmds.len().try_into().unwrap(),
            send_time,
            challenge,
            cmds,
            ..Default::default()
        }
    }

    pub fn _serialize(&self) -> Vec<u8> {
        let total_len = self.cmds.iter().map(|x| x.len()).sum::<u32>() + 12;
        let mut ret = vec![0; total_len as usize];
        ret[0x0..0x2].copy_from_slice(&self._unused.to_be_bytes());
        ret[0x2] = self.aux_property.into();
        ret[0x3] = self.cmd_count;
        ret[0x4..0x8].copy_from_slice(&self.send_time.to_be_bytes());
        ret[0x8..0xc].copy_from_slice(&self.challenge.to_be_bytes());
        let mut cur = 0xc;

        for cmd in &self.cmds {
            if let Some(bytes) = cmd._serialize() {
                ret[cur..cur+cmd.len() as usize].copy_from_slice(&bytes);
            }
            cur += cmd.len() as usize;
        }

        ret
    }
}

impl From<Vec<u8>> for Reply {
    fn from(buf: Vec<u8>) -> Self {
        let mut ret = Self {
            _unused: u16::from_be_bytes(buf[0..2].try_into().unwrap()),
            aux_property: AuxillaryProperty::from(buf[2]),
            cmd_count: buf[3],
            send_time: u32::from_be_bytes(buf[0x4..0x8].try_into().unwrap()),
            challenge: u32::from_be_bytes(buf[0x8..0xc].try_into().unwrap()),
            cmds: Vec::new(),
            ..Default::default()
        };

        let mut offset = 0xc;
        while offset < buf.len() {
            let cmd= Command::deserialize(&buf, offset);
            offset += cmd.len() as usize;
            ret.cmds.push(cmd);
        }

        ret
    }
}