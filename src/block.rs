extern crate chrono;
extern crate ring;

use self::ring::digest::{digest, SHA256};
use self::chrono::offset::Utc;
use std::fmt;
use std::fmt::Write;

pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub proof: u64,
    pub previous_hash: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Block {
    pub fn new(data: &str, previous_hash: Vec<u8>) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            data: data.as_bytes().to_vec(),
            proof: 0,
            previous_hash: previous_hash,
            hash: Vec::new(),
        };

        block.set_hash();

        block
    }

    pub fn new_genesis_block() -> Block {
        Block::new("Genesis Block", vec![])
    }

    fn set_hash(&mut self) {
        let header = [
            &self.previous_hash[..],
            &self.data[..],
            &self.timestamp.to_string().as_bytes(),
        ].concat();
        self.hash = digest(&SHA256, &header[..]).as_ref().to_vec();
    }

    fn calculate_hash(&self) -> String {
        unimplemented!()
    }

    fn serialize() {
        unimplemented!()
    }

    fn hash_transactions() {
        unimplemented!()
    }

    pub fn get_prev_hash(&self) -> String {
        byte_to_string(&self.previous_hash)
    }

    pub fn get_hash(&self) -> String {
        byte_to_string(&self.hash)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Prev. Hash:\t{}\nData: \t\t{}\nHash: \t\t{}",
            self.get_prev_hash(),
            String::from_utf8(self.data.clone()).unwrap(),
            self.get_hash()
        )
    }
}

fn byte_to_string(input: &Vec<u8>) -> String {
    let mut hash = String::new();
    for byte in input {
        write!(&mut hash, "{:X}", byte).expect("Unable to write");
    }

    hash
}

fn deserialize_block() {
    unimplemented!()
}

struct ProofOfWork {}

impl ProofOfWork {
    fn new() {}
    fn prepare_data() {}
    fn run() {}
    fn validate() {}
}
