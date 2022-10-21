use crate::typ::CommandType;

#[derive(Debug, Clone, Copy)]
pub struct Connect {
    pub mtu: u16,
    pub channel_count: u8,
    pub len: u32
}

#[derive(Debug, Clone, Copy)]
pub enum CommandPayload {
    Connect(Connect)
}

impl CommandPayload {
    pub fn deserialize(buf: &[u8], typ: CommandType) -> Self {
        match typ {
            CommandType::Connect => {
                let mtu = u16::from_be_bytes(buf[2..4].try_into().unwrap());
                let channel_count = buf[11];
                Self::Connect(Connect { mtu, channel_count, len: 32 })
            },
            _ => panic!("Not implemented {:?}", typ)
        }
    }

    pub fn serialize(&self) -> Vec<u8> { Vec::new() }
}
