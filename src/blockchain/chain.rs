use super::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self
        .chain
        .last()
        .map_or(String::from("0"), |block| block.hash.clone());
        let block = Block::new(self.chain.len() as u32, data, previous_hash);
        self.chain.push(block);
    }
}