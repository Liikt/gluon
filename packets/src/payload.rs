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
    pub fn deserialize(_buf: &[u8]) -> Self {
        Self::Connect(Connect { mtu: 0, channel_count: 0, len: 32 })
    }

    pub fn serialize(&self) -> Vec<u8> { Vec::new() }
}
