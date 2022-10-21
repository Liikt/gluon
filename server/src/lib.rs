use std::net::{UdpSocket, SocketAddr};

use packets::request::Request;
use packets::reply::Reply;

static mut CTR: u32 = 0;

pub fn handle_request(buf: &[u8], _socket: &UdpSocket, _conn: SocketAddr) {
    let packet = Request::from(Vec::from(buf));
    println!("{} {:?}", unsafe { CTR }, packet);
    unsafe { CTR += 1 };
}

pub fn parse_packets() {
    let data = std::fs::read("./tmp/foo.conv").unwrap();
    let lines: Vec<&[u8]> = data.split(|x| *x == 0x15).filter(|x| x.len() > 0).collect();
    for p in lines {
        if p[0] == 0 {
            println!("cli => srv: {:?}", Request::from(Vec::from(&p[1..])));
        } else {
            println!("srv => cli: {:?}", Reply::from(Vec::from(&p[1..])));
        }
    }
}