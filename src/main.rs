use std::net::UdpSocket;
use std::env::args;
use std::process::exit;

use server::{handle_request, parse_packets};

fn main() {
    if args().len() > 1 && args().len() == 3 {
        parse_packets();
        exit(0);
    } else if args().len() != 1 {
        println!("Usage: {} <file path> <seperator byte>", args().next().unwrap());
        exit(1);
    }

    let socket = UdpSocket::bind("0.0.0.0:5055").unwrap();

    loop {
        let mut buf = [0; 0x38];
        let (amt, conn) = socket.recv_from(&mut buf).unwrap();

        if amt == 0x38 {
            handle_request(&buf, &socket, conn);
        }
    }
}
