use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotonCode {
    ClientKey,
    ModeKey,
    ServerKey,
    InitEncryption,
    Ping,
    Ok,
}

impl Into<u8> for PhotonCode {
    fn into(self) -> u8 {
        match self {
            Self::ClientKey => 1,
            Self::ModeKey => 2,
            Self::ServerKey => 1,
            Self::InitEncryption => 0, 
            Self::Ping => 1,
            Self::Ok => 0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Init,
    InitResponse,
    Operation,
    OperationResponse,
    Event,
    InternalOperationRequest,
    InternalOperationResponse,
    Message,
    RawMessage
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Init,
            1 => Self::InitResponse,
            2 => Self::Operation,
            3 => Self::OperationResponse,
            4 => Self::Event,
            6 => Self::InternalOperationRequest,
            7 => Self::InternalOperationResponse,
            8 => Self::Message,
            9 => Self::RawMessage,
            _ => panic!("No such Message Type {}", value)
        }
    }
}

impl Into<u8> for MessageType {
    fn into(self) -> u8 {
        match self {
            Self::Init => 0,
            Self::InitResponse => 1,
            Self::Operation => 2,
            Self::OperationResponse => 3,
            Self::Event => 4,
            Self::InternalOperationRequest => 6,
            Self::InternalOperationResponse => 7,
            Self::Message => 8,
            Self::RawMessage => 9,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpType {
    Array,
    Boolean,
    Byte,
    ByteArray,
    ObjectArray,
    Short,
    Float,
    Dictionary,
    Double,
    Hashtable,
    Integer,
    IntegerArray,
    Long,
    String,
    StringArray,
    Custom,
    Null,
    EventData,
    OperationRequest,
    OperationResponse,

    Unknown
}

impl From<u8> for GpType {
    fn from(value: u8) -> Self {
        match value {
            121 => Self::Array,
            111 => Self::Boolean,
            98  => Self::Byte,
            120 => Self::ByteArray,
            122 => Self::ObjectArray,
            107 => Self::Short,
            102 => Self::Float,
            68  => Self::Dictionary,
            100 => Self::Double,
            104 => Self::Hashtable,
            105 => Self::Integer,
            110 => Self::IntegerArray,
            108 => Self::Long,
            115 => Self::String,
            97  => Self::StringArray,
            99  => Self::Custom,
            42  => Self::Null,
            101 => Self::EventData,
            113 => Self::OperationRequest,
            112 => Self::OperationResponse,
        
            _ => Self::Unknown
        }
    }
}

impl Into<u8> for GpType {
    fn into(self) -> u8 {
        match self {
            Self::Array => 121,
            Self::Boolean => 111,
            Self::Byte => 98,
            Self::ByteArray => 120,
            Self::ObjectArray => 122,
            Self::Short => 107,
            Self::Float => 102,
            Self::Dictionary => 68,
            Self::Double => 100,
            Self::Hashtable => 104,
            Self::Integer => 109,
            Self::IntegerArray => 110,
            Self::Long => 108,
            Self::String => 115,
            Self::StringArray => 97,
            Self::Custom => 99,
            Self::Null => 42,
            Self::EventData => 101,
            Self::OperationRequest => 113,
            Self::OperationResponse => 112,
        
            Self::Unknown => 0xff
        }
    }
}

impl GpType {
    fn parse(&self, buf: &[u8], cur: &mut usize) -> Value {
        match self {
            Self::Array => todo!("Array not yet implemented"),
            Self::Boolean => todo!("Boolean not yet implemented"),
            Self::Byte => todo!("Byte not yet implemented"),
            Self::ByteArray => {
                let num = u32::from_be_bytes(buf[*cur..*cur+4].try_into()
                    .unwrap()) as usize;
                *cur += 4;
                let ret = Value::ByteArray(Vec::from(&buf[*cur..*cur+num]));
                *cur += num;
                ret
            },
            Self::ObjectArray => todo!("ObjectArray not yet implemented"),
            Self::Short => todo!("Short not yet implemented"),
            Self::Float => todo!("Float not yet implemented"),
            Self::Dictionary => todo!("Dictionary not yet implemented"),
            Self::Double => todo!("Double not yet implemented"),
            Self::Hashtable => todo!("Hashtable not yet implemented"),
            Self::Integer => {
                let ret = Value::Integer(
                    i32::from_be_bytes(buf[*cur..*cur+4].try_into().unwrap()));
                *cur += 4;
                ret
            },
            Self::IntegerArray => todo!("IntegerArray not yet implemented"),
            Self::Long => todo!("Long not yet implemented"),
            Self::String => {
                let num = u16::from_be_bytes(buf[*cur..*cur+2].try_into()
                    .unwrap()) as usize;
                *cur += 2;
                if num == 0 { return Value::String(String::from("")); }
                let mut string = String::from_utf8(
                    Vec::from(&buf[*cur..*cur+num])).unwrap();
                string.retain(|c| c != '\0');
                *cur += num;
                Value::String(string)
            },
            Self::StringArray => todo!("StringArray not yet implemented"),
            Self::Custom => todo!("Custom not yet implemented"),
            Self::Null => todo!("Null not yet implemented"),
            Self::EventData => todo!("EventData not yet implemented"),
            Self::OperationRequest => todo!("OperationRequest not yet implemented"),
            Self::OperationResponse => todo!("OperationResponse not yet implemented"),
            Self::Unknown => panic!("Tried to parse unknown :(")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Dictionary,
    StringArray(Vec<String>),
    Byte(u8),
    Custom,
    Double(f64),
    EventData,
    Float(f32),
    HashTable,
    Integer(i32),
    Short(i16),
    Long(i64),
    IntArray(Vec<i32>),
    Boolean(bool),
    OperationResponse,
    OperationRequest,
    String(String),
    ByteArray(Vec<u8>),
    Array,
    ObjectArray,
}

#[derive(Clone)]
pub struct Init {
    protocol_version: [u8; 2],
    client_sdk_id: u8,
    client_version: [u8; 4],
    app_id: String
}

impl std::fmt::Debug for Init {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Init[ protocol_version: {}.{}, client_sdk_id: {}, client_version: {}.{}.{}.{}, app_id: {} ]",
            self.protocol_version[0], self.protocol_version[1], 
            self.client_sdk_id, self.client_version[0], self.client_version[1],
            self.client_version[2], self.client_version[3], self.app_id
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InitResponse {
    acked_num: u8
}

#[derive(Debug, Clone)]
pub struct Operation {
    msg_type: MessageType,
    opcode: u8,
    values: BTreeMap<u8, Value>
}

#[derive(Debug, Clone)]
pub struct Event {
    msg_type: MessageType,
    opcode: u8,
    values: BTreeMap<u8, Value>
}

#[derive(Debug, Clone)]
pub struct InternalOperationRequest {
    msg_type: MessageType,
    opcode: u8,
    values: BTreeMap<u8, Value>
}

#[derive(Debug, Clone)]
pub struct InternalOperationResponse {
    msg_type: MessageType,
    opcode: u8,
    values: BTreeMap<u8, Value>
}

#[derive(Debug, Clone)]
pub enum PhotonCommand {
    Init(Init),
    InitResponse(InitResponse),
    Operation(Operation),
    Event(Event),
    InternalOperationRequest(InternalOperationRequest),
    InternalOperationResponse(InternalOperationResponse)
}

impl From<&[u8]> for PhotonCommand {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(buf[0], 0xf3);
        let msg_type = MessageType::from(buf[1]);

        match msg_type {
            MessageType::Init => {
                let protocol_version = [buf[2], buf[3]];
                let client_sdk_id = buf[4] >> 1;
                let client_version = [buf[5] >> 4, buf[5] & ((1 << 4) - 1),
                    buf[6], buf[7]];
                let mut app_id = String::from_utf8(Vec::from(&buf[9..]))
                    .unwrap();
                app_id.retain(|c| c != '\0');
                return Self::Init(Init { protocol_version, client_sdk_id,
                    client_version, app_id });
            },
            MessageType::InitResponse => {
                let num = buf[2];
                return Self::InitResponse(InitResponse { acked_num: num });
            },
            MessageType::Operation | MessageType::InternalOperationRequest |
            MessageType::InternalOperationResponse | MessageType::Event => {}
            _ => todo!("Not yet implemented: {:?} {:?}", msg_type, buf)
        }

        let opcode = buf[2];
        let num_obj = u16::from_be_bytes(buf[3..5].try_into().unwrap());
        let mut values: BTreeMap<u8, Value> = BTreeMap::new();
        let mut cur = 5;
        for _ in 0..num_obj {
            let key = buf[cur];
            let typ = GpType::from(buf[cur+1]);
            cur += 2;
            let val = typ.parse(buf, &mut cur);
            values.insert(key, val);
        }

        match msg_type {
            MessageType::Operation => {
                Self::Operation(Operation { 
                    msg_type, opcode, values })
            },
            MessageType::InternalOperationRequest => {
                Self::InternalOperationRequest(InternalOperationRequest { 
                    msg_type, opcode, values })
            },
            MessageType::InternalOperationResponse => {
                Self::InternalOperationResponse(InternalOperationResponse { 
                    msg_type, opcode, values })
            },
            MessageType::Event => {
                Self::Event(Event { 
                    msg_type, opcode, values })
            },
            _ => todo!("Not yet implemented: {:?}", msg_type)
        }

    }
}