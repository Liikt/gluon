use crate::typ::CommandType;
use crate::photon::PhotonCommand;
use crate::header::{get_fragment_map, CommandHeader};

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

#[derive(Debug, Clone, Copy)]
pub struct Ping {
    len: u32
}

#[derive(Debug, Clone, Copy)]
pub struct Disconnect {
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

#[derive(Debug, Clone)]
pub struct Fragmented {
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
    Ping(Ping),
    Disconnect(Disconnect),
    Reliable(Reliable),
    Unreliable(Unreliable),
    Fragmented(Fragmented),
    ServerTime(ServerTime)
}

impl CommandPayload {
    pub fn deserialize(buf: &[u8], header: CommandHeader) -> Option<Self> {
        match header.cmd_type {
            CommandType::None => {
                Some(Self::None)
            },
            CommandType::Connect => {
                let mtu = u16::from_be_bytes(buf[2..4].try_into().unwrap());
                let channel_count = buf[11];
                Some(Self::Connect(Connect { mtu, channel_count, len: 32 }))
            },
            CommandType::Ack => {
                let acked_seq_num = u32::from_be_bytes(buf[0..4].try_into()
                    .unwrap());
                let send_time = u32::from_be_bytes(buf[4..8].try_into()
                    .unwrap());
                Some(Self::Ack(Ack { acked_seq_num, send_time, len: 8}))
            },
            CommandType::VerifyConnect => {
                let peer_id = u16::from_be_bytes(buf[0..2].try_into()
                    .unwrap());
                Some(Self::VerifyConnect(VerifyConnect { peer_id, len: 32 }))
            },
            CommandType::Disconnect => {
                Some(Self::Ping(Ping { len: 12 }))
            },
            CommandType::Ping => {
                Some(Self::Ping(Ping { len: 12 }))
            },
            CommandType::Reliable => {
                let size = buf.len() as u32;
                let payload = PhotonCommand::from(buf);
                Some(Self::Reliable(Reliable { payload, len: 12 + size }))
            },
            CommandType::Unreliable => {
                let size = buf.len() as u32;
                let payload = PhotonCommand::from(buf);
                Some(Self::Unreliable(Unreliable { payload, len: 16 + size }))
            },
            CommandType::Fragmented => {
                let mut map = get_fragment_map();
                let mut fragment = map.get_mut(&header.start_seq_num.unwrap())
                    .expect("Sequence number not found :/");

                let size = buf.len();
                let start = header.fragment_offset.unwrap() as usize;
                fragment.bytes[start..start+size].copy_from_slice(&buf);

                fragment.received_fragments += 1;
                if fragment.received_fragments != fragment.total_fragments {
                    return None;
                }

                let size = fragment.bytes.len() as u32;
                let payload = PhotonCommand::from(&*fragment.bytes);
                map.remove(&header.start_seq_num.unwrap());
                Some(Self::Fragmented(Fragmented { payload, len: 32 + size }))
            }
            CommandType::ServerTime => {
                Some(Self::ServerTime(ServerTime { len: 12 }))
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> { Vec::new() }

    pub fn len(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Connect(p) => p.len,
            Self::Ack(p) => p.len,
            Self::VerifyConnect(p) => p.len,
            Self::Disconnect(p) => p.len,
            Self::Ping(p) => p.len,
            Self::Reliable(p) => p.len,
            Self::Unreliable(p) => p.len,
            Self::Fragmented(p) => p.len,
            Self::ServerTime(p) => p.len,
        }
    }
}
