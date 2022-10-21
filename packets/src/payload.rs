use crate::typ::CommandType;

#[derive(Debug, Clone, Copy)]
pub struct Connect {
    pub mtu: u16,
    pub channel_count: u8,
    len: u32
}

#[derive(Debug, Clone, Copy)]
pub struct Ack {
    pub acked_seq_num: u32,
    pub send_time: u32,
    len: u32
}

#[derive(Debug, Clone, Copy)]
pub enum CommandPayload {
    Connect(Connect),
    Ack(Ack)
}

impl CommandPayload {
    pub fn deserialize(buf: &[u8], typ: CommandType) -> Self {
        match typ {
            CommandType::Connect => {
                let mtu = u16::from_be_bytes(buf[2..4].try_into().unwrap());
                let channel_count = buf[11];
                Self::Connect(Connect { mtu, channel_count, len: 32 })
            },
            CommandType::Ack => {
                let acked_seq_num = u32::from_be_bytes(buf[0..4].try_into()
                    .unwrap());
                let send_time = u32::from_be_bytes(buf[4..8].try_into()
                    .unwrap());
                Self::Ack(Ack { acked_seq_num, send_time, len: 8})
            },
            _ => panic!("Not implemented {:?}", typ)
        }
    }

    pub fn serialize(&self) -> Vec<u8> { Vec::new() }

    pub fn len(&self) -> u32 {
        match self {
            Self::Connect(p) => p.len,
            Self::Ack(p) => p.len,
            Self::PeerID(p) => p.len
        }
    }
}
