use std::{env, fs};

enum BencodeDataType {
    String,
    Integer,
    List,
    Dictionary,
    Unknown,
}


impl BencodeDataType {
    fn from_identifier(id: char) -> Self {
        match id {
            's' => BencodeDataType::String,
            'i' => BencodeDataType::String,
            'l' => BencodeDataType::String,
            'd' => BencodeDataType::String,
            _ => BencodeDataType::Unknown
        }
    }
}

fn bencode(bytes: &Vec<u8>) {
    let mut has_type = false;
    let mut pos: usize = 0;


    if !has_type {
        let t = BencodeDataType::from_identifier(bytes[pos] as char);
        match t {
            BencodeDataType::Unknown => {
                println!("Type unknown");
                return;
            }
            other => {
                println!("Match");
            }
        }

        pos += 1;
        has_type = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    dbg!(file_name);

    // TODO: All bytes are in memory now. Maybe we can add lazy reading from file using some sort of Reader abstraction?
    let bytes = fs::read(file_name)
        .expect(&format!("Unable to read file: {}", file_name));

    println!("Size: {} bytes", bytes.len());

    bencode(&bytes);
}
