use serde::{Serialize, Deserialize};
use blake3::Hasher;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine_block();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Hasher::new();
        let input = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);
        hasher.update(input.as_bytes());
        hasher.finalize().to_hex().to_string()
    }

    pub fn mine_block(&mut self) {
        const DIFFICULT: &str = "0000";
        while !self.hash.starts_with(DIFFICULT) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block minded with hash{}", self.hash)
    }
}
