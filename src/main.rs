use crate::bencode::BencodeParser;
use crate::bittorrent::BitTorrent;
use std::{env, fs};

mod bencode;
mod bittorrent;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    dbg!(file_name);

    // TODO: All bytes are in memory now. Maybe we can add lazy reading from file using some sort of Reader abstraction?
    let bytes = fs::read(file_name).expect(&format!("Unable to read file: {}", file_name));

    let mut p = BencodeParser::new(bytes);

    let b = p.parse();

    let client = BitTorrent::from_bencode(&b, p.info_hash);

    let body = client.server_call().unwrap();

    println!("{}", body);
}
