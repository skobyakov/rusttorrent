use crate::bencode::Bencode;
use std::str::from_utf8;

// TODO: strings could be replaced with `&str` and lifetime annotation
// TODO: as well as `&i64`, as well as `&Vec<u8>`
#[derive(Debug)]
pub struct BitTorrent {
    pub announce: String,
    pub name: String,
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    info_hash: [u8; 160],
    files: Vec<(i64, String)>,
}

impl BitTorrent {
    // TODO: Should we use `[u8;160] without borrowing? Or referencing?
    pub fn from_bencode(bencode: &Bencode, info_hash: [u8; 160]) -> Self {
        let mut res = BitTorrent {
            announce: String::from(""),
            name: "".to_string(),
            piece_length: 0,
            pieces: vec![],
            info_hash,
            files: vec![],
        };

        // TODO Refactor parse errors
        match bencode {
            Bencode::Dictionary(dict) => {
                for (key, val) in dict {
                    let key_str = from_utf8(key).expect("invalid UTF-8");

                    match key_str {
                        "announce" => {
                            match val {
                                Bencode::Bytes(bytes) => {
                                    res.announce =
                                        from_utf8(bytes).expect("invalid UTF-8").to_owned();
                                }
                                _ => {} // TODO: Error
                            }
                        }
                        "info" => {
                            match val {
                                Bencode::Dictionary(dict) => {
                                    for (key, val) in dict {
                                        let key_str = from_utf8(key).expect("invalid UTF-8");

                                        match key_str {
                                            "name" => {
                                                match val {
                                                    Bencode::Bytes(bytes) => {
                                                        res.name = from_utf8(bytes)
                                                            .expect("invalid UTF-8")
                                                            .to_owned();
                                                    }
                                                    _ => {} // TODO: Error
                                                }
                                            }
                                            "piece length" => {
                                                match val {
                                                    Bencode::Integer(i) => {
                                                        res.piece_length = i.to_owned()
                                                    }
                                                    _ => {} // TODO: Error
                                                }
                                            }
                                            "pieces" => {
                                                match val {
                                                    Bencode::Bytes(bytes) => {
                                                        res.pieces = bytes.to_owned()
                                                    }
                                                    _ => {} // TODO: Error
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {} // TODO: Error
                            }
                        }
                        _ => {} // TODO: Implement
                    }
                }
            }
            _ => {} // TODO: Implement
        }

        res
    }
}
