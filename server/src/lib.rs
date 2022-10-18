use std::net::{UdpSocket, SocketAddr};

use packets::*;

pub fn handle_request(buf: &[u8], socket: &UdpSocket, conn: SocketAddr) {
    let packet = CommandPacket::from(Vec::from(buf));
    println!("{:?}", packet);
    let header = 
        CommandHeader::new(
            ClientCommandType::Ack, 
            packet.cmds[0].header.channel_id,
            packet.cmds[0].header.flags,
            packet.cmds[0].header.reliable_seq_num + 1
        );
    let mut data = vec![0; 8];
    data[0..4].copy_from_slice(&packet.cmds[0].header.reliable_seq_num.to_be_bytes());
    data[4..8].copy_from_slice(&packet.time.to_be_bytes());
    let cmds = vec![Command::new(header, data)];
    let ret_packet = Reply::new(packet.time + 0x10, packet.challenge, cmds);
    socket.send_to(&ret_packet.serialize(), conn).unwrap();
    socket.send_to(&ret_packet.serialize(), conn).unwrap();
}