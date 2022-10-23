use std::net::{UdpSocket, SocketAddr};
use std::env::args;

mod request;
use crate::request::Request;

mod reply;
use crate::reply::Reply;

static mut CTR: u32 = 0;

pub fn handle_request(buf: &[u8], _socket: &UdpSocket, _conn: SocketAddr) {
    let packet = Request::from(Vec::from(buf));
    println!("{} {:?}", unsafe { CTR }, packet);
    unsafe { CTR += 1 };
}

pub fn parse_packets() {
    let data = std::fs::read(args().nth(1).unwrap()).unwrap();
    let lines: Vec<&[u8]> = data
        .split(|x| *x == u8::from_str_radix(&*args().nth(2).unwrap(), 16).unwrap())
        .filter(|x| x.len() > 0).collect();
    for p in lines {
        if p[0] == 0 {
            println!("cli => srv: {:?}", Request::from(Vec::from(&p[1..])));
        } else {
            println!("srv => cli: {:?}", Reply::from(Vec::from(&p[1..])));
        }
    }
}