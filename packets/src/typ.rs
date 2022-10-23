#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    None,
    Ack,
    Connect,
    VerifyConnect,
    Disconnect,
    Ping,
    Reliable,
    Unreliable,
    Fragmented,
    ServerTime,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::None
    }
}

impl From<u8> for CommandType {
    fn from(val: u8) -> Self {
        match val {
            0  => Self::None, 
            1  => Self::Ack,
            2  => Self::Connect,
            3  => Self::VerifyConnect,
            4  => Self::Disconnect,
            5  => Self::Ping,
            6  => Self::Reliable,
            7  => Self::Unreliable,
            8  => Self::Fragmented,
            12 => Self::ServerTime,

            _ => panic!("{:x} is not a valid command tye", val)
        }
    }
}

impl Into<u8> for CommandType {
    fn into(self) -> u8 {
        match self {
            Self::None          => 0,
            Self::Ack           => 1,
            Self::Connect       => 2,
            Self::VerifyConnect => 3,
            Self::Disconnect    => 4,
            Self::Ping          => 5,
            Self::Reliable      => 6,
            Self::Unreliable    => 7,
            Self::Fragmented    => 8,
            Self::ServerTime    => 12,
        }
    }
}