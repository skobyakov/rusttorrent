use std::collections::BTreeMap;
use std::str::from_utf8;
use std::{env, fs};

#[derive(Debug)]
enum Bencode {
    Bytes(Vec<u8>),
    Integer(i64),
    List(Vec<Bencode>),
    Dictionary(BTreeMap<Vec<u8>, Bencode>),
}

struct BencodeParser {
    input: Vec<u8>,
    pos: usize,
}

impl BencodeParser {
    fn new(input: Vec<u8>) -> Self {
        BencodeParser { input, pos: 0 }
    }

    // TODO: Maybe we can stop returning new Vec? Try to return slice of original vector
    fn parse_bytes(&mut self) -> Vec<u8> {
        let mut offset = 0;

        while self.input[self.pos + offset] != ':' as u8 {
            offset += 1;
        }

        let s = &self.input[self.pos..self.pos + offset];
        let num_bytes: usize = from_utf8(&s)
            .expect("invalid UTF-8")
            .parse()
            .expect("invalid number");

        self.pos += offset + 1;

        let bytes = &self.input[self.pos..self.pos + num_bytes];

        self.pos += num_bytes;

        bytes.to_vec()
    }

    fn parse_int(&mut self) -> i64 {
        self.pos += 1;
        let mut offset = 0;

        while self.input[self.pos + offset] != 'e' as u8 {
            offset += 1
        }

        let s = &self.input[self.pos..self.pos + offset];

        self.pos += offset;

        from_utf8(&s)
            .expect("invalid UTF-8")
            .parse()
            .expect("invalid number")
    }

    fn parse_list(&mut self) -> Vec<Bencode> {
        self.pos += 1;
        let mut res: Vec<Bencode> = vec![];

        while self.input[self.pos] != 'e' as u8 {
            res.push(self.parse());
        }

        self.pos += 1;

        res
    }

    fn parse_dictionary(&mut self) -> BTreeMap<Vec<u8>, Bencode> {
        self.pos += 1;
        let mut res = BTreeMap::new();

        while self.input[self.pos] != 'e' as u8 {
            let key = self.parse_bytes();
            let value = self.parse();

            res.insert(key, value);
        }

        self.pos += 1;

        res
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

    let test_bytes = "d3:cow3:moo4:spam4:eggse".as_bytes().to_vec();
    let mut p = BencodeParser::new(test_bytes);
    let b = p.parse();
    dbg!(b);
}
