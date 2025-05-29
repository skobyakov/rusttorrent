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
    pub info_hash: String,
    pub files: Vec<(i64, String)>,
}

impl BitTorrent {
    pub fn from_bencode(bencode: &Bencode, info_hash: String) -> Self {
        let mut res = BitTorrent {
            announce: String::from(""),
            name: "".to_string(),
            piece_length: 0,
            pieces: vec![],
            info_hash,
            files: vec![],
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

    pub fn server_call(&self) {
        // TODO: Implement me
        // http://bt3.t-ru.org/ann?info_hash=%F3%8Fs%1BU~%BCL%88d%D4%17%17-%C5%D5%0B%0E%17%C5&peer_id=kHVvUGob3rYADHlg1Zi5&ip=5.167.242.248&uploaded=0&downloaded=0&left=15066515&port=6881
    }
}
