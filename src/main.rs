use std::net::UdpSocket;

use server::handle_request;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:5055").unwrap();

    loop {
        let mut buf = [0; 0x38];
        let (amt, conn) = socket.recv_from(&mut buf).unwrap();

        if amt == 0x38 {
            handle_request(&buf, &socket, conn);
        }
    }
}
