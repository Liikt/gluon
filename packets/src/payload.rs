use crate::typ::CommandType;
use crate::photon::PhotonCommand;

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
pub struct VerifyConnect {
    pub peer_id: u16,
    len: u32
}

#[derive(Debug, Clone)]
pub struct Reliable {
    pub payload: PhotonCommand,
    len: u32
}

#[derive(Debug, Clone)]
pub struct Unreliable {
    pub payload: PhotonCommand,
    len: u32
}

#[derive(Debug, Clone, Copy)]
pub struct ServerTime {
    len: u32
}

#[derive(Debug, Clone)]
pub enum CommandPayload {
    None,
    Ack(Ack),
    Connect(Connect),
    VerifyConnect(VerifyConnect),
    Reliable(Reliable),
    Unreliable(Unreliable),
    ServerTime(ServerTime)
}

impl CommandPayload {
    pub fn deserialize(buf: &[u8], typ: CommandType) -> Self {
        match typ {
            CommandType::None => {
                Self::None
            },
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
            CommandType::VerifyConnect => {
                let peer_id = u16::from_be_bytes(buf[0..2].try_into()
                    .unwrap());
                Self::VerifyConnect(VerifyConnect { peer_id, len: 32 })
            },
            CommandType::Reliable => {
                let size = buf.len() as u32;
                let payload = PhotonCommand::from(buf);
                Self::Reliable(Reliable { payload, len: 12 + size })
            },
            CommandType::Unreliable => {
                let size = buf.len() as u32;
                let payload = PhotonCommand::from(buf);
                Self::Unreliable(Unreliable { payload, len: 16 + size })
            },
            CommandType::ServerTime => {
                Self::ServerTime(ServerTime { len: 12 })
            }
            _ => panic!("Not implemented {:?} {:?}", typ, buf)
        }
    }

    pub fn serialize(&self) -> Vec<u8> { Vec::new() }

    pub fn len(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Connect(p) => p.len,
            Self::Ack(p) => p.len,
            Self::VerifyConnect(p) => p.len,
            Self::Reliable(p) => p.len,
            Self::Unreliable(p) => p.len,
            Self::ServerTime(p) => p.len,
        }
    }
}
