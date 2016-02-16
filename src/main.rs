extern crate ipfs;

use std::net::TcpStream;
use std::io::{BufRead, BufReader};

use ipfs::crypto::public_key::PublicKey;
use ipfs::net::secio::SecureStream;

fn main() {
    println!("Testing secio connection to 127.0.0.1:4001");
    let stream = TcpStream::connect("127.0.0.1:4001").unwrap();
    let pub_key = PublicKey::generate(1024);
    let stream = SecureStream::new(stream, pub_key).unwrap();
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{:?}", line.unwrap());
    }
}
