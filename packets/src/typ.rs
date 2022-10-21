#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    Ack,
    Connect,
    PeerID,
    Disconnect,
    Unreliable,
    Fragmented,
    NotImplemented(u8)
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::NotImplemented(0xff)
    }
}

impl From<u8> for CommandType {
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

impl Into<u8> for CommandType {
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