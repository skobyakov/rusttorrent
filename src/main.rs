use std::collections::BTreeMap;
use std::{env, fs};

// TODO: Do I really need full bencode parser?
enum Bencode {
    Bytes(Vec<u8>),
    Integer(i64),
    List(Vec<Bencode>),
    Dictionary(BTreeMap<String, Bencode>),
}

struct BencodeParser {
    input: Vec<u8>,
    pos: usize,
}

impl BencodeParser {
    fn new(input: Vec<u8>) -> Self {
        BencodeParser { input, pos: 0 }
    }

    fn parse_bytes(&mut self) -> Vec<u8> {
        // TODO: Implement me
        return vec![1, 2, 3];
    }

    fn parse_int(&mut self) -> i64 {
        // TODO: Implement me
        return 1;
    }

    fn parse_list(&mut self) -> Vec<Bencode> {
        // TODO: Implement me
        return vec![Bencode::Integer(1)];
    }

    fn parse_dictionary(&mut self) -> BTreeMap<String, Bencode> {
        // TODO: Implement me
        return BTreeMap::new();
    }

    fn parse(&mut self) -> Bencode {
        let char = self.input[self.pos] as char;
        match char {
            'i' => Bencode::Integer(self.parse_int()),
            'l' => Bencode::List(self.parse_list()),
            'd' => Bencode::Dictionary(self.parse_dictionary()),
            _ => Bencode::Bytes(self.parse_bytes()),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    dbg!(file_name);

    // TODO: All bytes are in memory now. Maybe we can add lazy reading from file using some sort of Reader abstraction?
    let bytes = fs::read(file_name).expect(&format!("Unable to read file: {}", file_name));

    println!("Size: {} bytes", bytes.len());

    let mut p = BencodeParser::new(bytes);
    let b = p.parse();
}
