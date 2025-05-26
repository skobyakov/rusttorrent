use crate::bencode::BencodeParser;
use std::{env, fs};

pub mod bencode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    dbg!(file_name);

    // TODO: All bytes are in memory now. Maybe we can add lazy reading from file using some sort of Reader abstraction?
    let bytes = fs::read(file_name).expect(&format!("Unable to read file: {}", file_name));

    let test_bytes = "d3:cow3:moo4:spami3ee".as_bytes().to_vec();
    let mut p = BencodeParser::new(test_bytes);
    let b = p.parse();
    dbg!(b);
}
