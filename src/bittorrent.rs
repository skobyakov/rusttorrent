use crate::bencode::Bencode;

#[derive(Debug)]
pub struct BitTorrent {
    announce: String,
    name: String,
    piece_length: i64,
    pieces: Vec<u8>,
    info_hash: [u8; 160],
    files: Vec<(i64, String)>,
}

impl BitTorrent {
    pub fn from_bencode(bencode: &Bencode) -> Self {
        BitTorrent {
            announce: String::from(""),
            name: "".to_string(),
            piece_length: 0,
            pieces: vec![],
            info_hash: [0u8; 160],
            files: vec![],
        }
    }
}
