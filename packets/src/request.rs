use std::fmt::Debug;

use crate::header::CommandHeader;
use crate::payload::CommandPayload;

#[derive(Clone)]
pub struct Command {
    pub header: CommandHeader,
    pub payload: CommandPayload
}

impl Command {
    pub fn new(mut header: CommandHeader, payload: CommandPayload) -> Self {
        header.size += payload.len();
        Self {
            header,
            payload
        }
    }

    pub fn deserialize(buf: &[u8], mut offset: usize) -> Self {
        let header = CommandHeader::deserialize(&buf, offset);
        offset += header.len() as usize;

        let payload_len = header.size as usize - header.len();
        let payload = CommandPayload::deserialize(
            &buf[offset..offset+payload_len], header.cmd_type);
        Self {
            header,
            payload
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ret = vec![0; self.len() as usize];
        ret[..self.header.len()].copy_from_slice(&self.header.serialize());
        ret[self.header.len()..].copy_from_slice(&self.payload.serialize());

        ret
    }

    pub fn len(&self) -> u32 {
        self.header.size
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cmd[ {:?}, payload: {:x?} ]", self.header, self.payload)
    }
}

pub struct Request {
    pub peer_id: u16,
    pub use_crc: bool,
    pub cmd_count: u8,
    pub time: u32,
    pub challenge: u32,
    pub cmds: Vec<Command>
}

impl From<Vec<u8>> for Request {
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

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet[ peer_id: 0x{:04x}, use_crc: {}, cmd_count: 0x{:x}, time: 0x{:08x}, challenge: 0x{:08x}, commands: {:?}] ]", 
            self.peer_id, self.use_crc, self.cmd_count, self.time, 
            self.challenge, self.cmds)
    }
}