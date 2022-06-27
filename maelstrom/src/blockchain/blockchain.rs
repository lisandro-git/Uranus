use super::block;
use crate::blockchain::block::{Block, Genesis, Hashing};
use std::ops::{Deref, DerefMut};


#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn new(genesis_block: Block) -> Blockchain {
        return Blockchain {
            blocks: vec![genesis_block],
        };
    }
    pub fn add_block(&mut self, mut Blk: Block) {
        // edode : calculating the previous Block's hash
        let last_block = Blk.calculate_hash(&self.blocks.last().unwrap());
        Blk.header.prev_block_hash = last_block;
        self.blocks.push(Blk);
    }
    pub fn get_last_block_hash(&self) -> String {
        return self
            .blocks
            .last()
            .unwrap()
            .clone()
            .header.prev_block_hash;
    }
}
impl Deref for Blockchain {
    type Target = Vec<block::Block>;
    fn deref(&self) -> &Self::Target {
        return &self.blocks;
    }
}
impl DerefMut for Blockchain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.blocks;
    }
}
