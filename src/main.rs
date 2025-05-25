use std::{env, fs};

enum BencodeDataType {
    String,
    Integer,
    List,
    Dictionary,
    None,
}

impl BencodeDataType {
    fn from_identifier(id: char) -> Self {
        match id {
            's' => BencodeDataType::String,
            'i' => BencodeDataType::String,
            'l' => BencodeDataType::String,
            'd' => BencodeDataType::String,
            _ => BencodeDataType::None,
        }
    }
}

fn bencode(bytes: &Vec<u8>) {
    let mut bencode_type = BencodeDataType::None;
    let mut pos: usize = 0;

    loop {
        if pos >= bytes.len() {
            break;
        }

        match bencode_type {
            BencodeDataType::None => {
                let t = BencodeDataType::from_identifier(bytes[pos] as char);
                match t {
                    BencodeDataType::None => {
                        println!("Invalid bencode data type at position {}", pos);
                        break;
                    }
                    other => bencode_type = other,
                }

                pos += 1;
            }
            BencodeDataType::Dictionary => {}
            BencodeDataType::String => {}
            BencodeDataType::Integer => {}
            BencodeDataType::List => {}
        }
    }

    println!("We are done");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    dbg!(file_name);

    // TODO: All bytes are in memory now. Maybe we can add lazy reading from file using some sort of Reader abstraction?
    let bytes = fs::read(file_name).expect(&format!("Unable to read file: {}", file_name));

    println!("Size: {} bytes", bytes.len());

    bencode(&bytes);
}
