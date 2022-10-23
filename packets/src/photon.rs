#[derive(Clone)]
pub struct AppId {
    prot_version: [u8; 2],
    client_sdk_version: u8,
    client_version: [u8; 4],
    app_id: String
}

impl std::fmt::Debug for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppID[ prot_version: {}.{}, client_sdk_version: {}, client_version: {}.{}.{}.{}, app_id: {} ]", 
            self.prot_version[0], self.prot_version[1], 
            self.client_sdk_version, self.client_version[0],
            self.client_version[1], self.client_version[2], 
            self.client_version[3], self.app_id
        )
    }
}

#[derive(Clone)]
pub struct Ack {
    ack: u8
}

impl std::fmt::Debug for Ack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ack[ ack: {} ]", self.ack)
    }
}

#[derive(Debug, Clone)]
pub enum PhotonCommand {
    AppId(AppId),
    Ack(Ack)
}

impl From<&[u8]> for PhotonCommand {
    fn from(buf: &[u8]) -> Self {
        assert_eq!(buf[0], 0xf3);
        match buf[1] {
            0 => {
                let prot_version = [buf[2], buf[3]];
                let client_sdk_version = buf[4];
                let client_version = [buf[5] >> 4, buf[5] & ((1 << 4) - 1),
                    buf[6], buf[7]];
                let mut app_id = String::from_utf8(Vec::from(&buf[9..]))
                    .unwrap();
                app_id.retain(|c| c != '\0');
                Self::AppId(AppId { 
                    prot_version, 
                    client_sdk_version,
                    client_version,
                    app_id 
                })
            },
            1 => {
                let ack = buf[2];
                Self::Ack(Ack { ack })
            },
            _ => panic!("not implemented or just big doo doo {:x?}", buf)
        }
    }
}