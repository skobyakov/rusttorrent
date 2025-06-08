use crate::bencode::Bencode;
use rand::{Rng, distr::Alphanumeric, rng};
use reqwest::blocking::get;
use std::io::Bytes;
use std::ops::Add;
use std::str::from_utf8;

// TODO: strings could be replaced with `&str` and lifetime annotation
// TODO: as well as `&i64`, as well as `&Vec<u8>`
#[derive(Debug)]
pub struct BitTorrent {
    pub announce: String,
    pub name: String,
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub info_hash: String,
    pub files: Vec<(i64, String)>,
    pub peer_id: String,
}

impl BitTorrent {
    pub fn from_bencode(bencode: &Bencode, info_hash: String) -> Self {
        let random_string: String = rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        let mut res = BitTorrent {
            announce: String::from(""),
            name: "".to_string(),
            piece_length: 0,
            pieces: vec![],
            info_hash,
            files: vec![],
            peer_id: random_string,
        };

        // TODO: Refactor errors
        if let Bencode::Dictionary(dict) = bencode {
            for (key, val) in dict {
                let key_str = from_utf8(key).expect("invalid UTF-8");
                match key_str {
                    "announce" => {
                        if let Bencode::Bytes(bytes) = val {
                            res.announce = from_utf8(bytes).expect("invalid UTF-8").to_owned();
                        } else {
                            panic!("announce key should be bytes");
                        }
                    }
                    "info" => {
                        if let Bencode::Dictionary(dict) = val {
                            for (key, val) in dict {
                                let key_str = from_utf8(key).expect("invalid UTF-8");

                                match key_str {
                                    "name" => {
                                        if let Bencode::Bytes(bytes) = val {
                                            res.name =
                                                from_utf8(bytes).expect("invalid UTF-8").to_owned();
                                        } else {
                                            panic!("name key should be bytes");
                                        }
                                    }
                                    "piece length" => {
                                        if let Bencode::Integer(i) = val {
                                            res.piece_length = i.to_owned()
                                        } else {
                                            panic!("piece length key should be integer");
                                        }
                                    }
                                    "pieces" => {
                                        if let Bencode::Bytes(b) = val {
                                            res.pieces = b.to_owned()
                                        } else {
                                            panic!("pieces value should be bytes");
                                        }
                                    }
                                    "files" => {
                                        if let Bencode::List(list) = val {
                                            for (_, v) in list.iter().enumerate() {
                                                if let Bencode::Dictionary(dict) = v {
                                                    let mut name = "";
                                                    let mut size = &0;
                                                    for (key, val) in dict {
                                                        let key_str =
                                                            from_utf8(key).expect("invalid UTF-8");

                                                        match key_str {
                                                            "length" => {
                                                                if let Bencode::Integer(i) = val {
                                                                    size = &i;
                                                                } else {
                                                                    panic!(
                                                                        "file length key should be integer"
                                                                    );
                                                                }
                                                            }
                                                            "path" => {
                                                                if let Bencode::List(l) = val {
                                                                    let first = l.get(0).unwrap();
                                                                    if let Bencode::Bytes(bytes) =
                                                                        first
                                                                    {
                                                                        let file = from_utf8(bytes)
                                                                            .expect(
                                                                                "invalid UTF-8",
                                                                            );
                                                                        name = file;
                                                                    } else {
                                                                        panic!(
                                                                            "file path key should be bytes"
                                                                        )
                                                                    }
                                                                } else {
                                                                    panic!(
                                                                        "file path key should be list"
                                                                    )
                                                                }
                                                            }
                                                            _ => {} // TODO: Error
                                                        }
                                                    }
                                                    res.files
                                                        .push((size.to_owned(), name.to_owned()));
                                                } else {
                                                    panic!("files item value should be dictionary");
                                                }
                                            }
                                        } else {
                                            panic!("files value should be list");
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            panic!("info key should be dictionary");
                        }
                    }
                    _ => {}
                }
            }
        } else {
            panic!("not a dictionary");
        }

        res
    }

    pub fn server_call(&self) -> Result<Vec<u8>, reqwest::Error> {
        let url = format!(
            "{}?info_hash={}&peer_id={}&ip=8.8.8.8&port=6881&uploaded=0&downloaded=0&left=1000",
            self.announce, self.info_hash, self.peer_id
        );

        let b = get(&url)?.bytes()?.to_vec();

        Ok(b)
    }
}
