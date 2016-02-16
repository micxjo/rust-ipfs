extern crate ipfs;

use std::io::{BufRead, BufReader};

use ipfs::crypto::public_key::PublicKey;
use ipfs::net::secio::SecureStream;
use ipfs::net::multiaddr::Multiaddr;

fn main() {
    println!("Testing secio connection to /ip4/127.0.0.1/4001");

    let addr = "/ip4/127.0.0.1/tcp/4001".parse::<Multiaddr>().unwrap();
    let pub_key = PublicKey::generate(1024);
    let stream = SecureStream::dial(&addr, pub_key).unwrap();

    for line in BufReader::new(stream).lines() {
        println!("{:?}", line.unwrap());
    }
}
