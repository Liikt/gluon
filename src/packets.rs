use std::fmt::Debug;

pub struct InitialConnection {
    peer_id: u16,
    use_crc: bool,
    cmd_count: u8,
    time: u32,
    challenge: u32,
    data: Vec<u8>
}

impl From<[u8; 0x38]> for InitialConnection {
    fn from(val: [u8; 0x38]) -> Self {
        Self { 
            peer_id: u16::from_be_bytes(val[0x0..0x2].try_into().unwrap()),
            use_crc: val[2] != 0,
            cmd_count: val[3],
            time: u32::from_be_bytes(val[0x4..0x8].try_into().unwrap()),
            challenge: u32::from_be_bytes(val[0x8..0xc].try_into().unwrap()),
            data: val[0xc..].iter().map(|x| *x).collect()
        }
    }
}

impl Debug for InitialConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet[ peer_id: 0x{:04x}, use_crc: {}, cmd_count: 0x{:x}, time: 0x{:08x}, challenge: 0x{:08x}, data: [0x{:02x}, 0x{:02x}, ... (0x{:x})] ]", 
            self.peer_id, self.use_crc, self.cmd_count, self.time, 
            self.challenge, self.data[0], self.data[1], self.data.len() - 2)
    }
}