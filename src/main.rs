
use std::net::UdpSocket;

mod packets;
use packets::InitialConnection;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:5055").unwrap();
    let mut ctr = 0;

    loop {
        let mut buf = [0; 0x38];
        let (amt, _) = socket.recv_from(&mut buf).unwrap();

        if amt == 0x38 {
            let packet = InitialConnection::from(buf);
            println!("{}: {:#02x?}", ctr, packet);
            ctr += 1;
        }
    }
}
