use std::net::{UdpSocket, SocketAddr};

use packets::*;

pub fn handle_request(buf: &[u8], socket: &UdpSocket, conn: SocketAddr) {
    let packet = CommandPacket::from(Vec::from(buf));
    println!("{:?}", packet);
    socket.send_to(&[0; 100], conn).unwrap();
}