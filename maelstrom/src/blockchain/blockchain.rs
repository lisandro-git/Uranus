use std::ptr::addr_of;
use super::block;
use sha2::{Sha512, Digest};
use bincode;
use crate::blockchain::block::{Block, Genesis};

pub trait Hashing {
    fn encode_block(&self) -> Vec<u8>;
    fn hash(&self) -> [u8; 32];
    fn get_last_block_hash(&self) -> [u8; 32];
}

pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}
impl Blockchain {
    pub fn new() -> Blockchain {
        return Blockchain {
            blocks: vec![Block::genesis_block()],
        };
    }
    pub fn add_block(&mut self, block: block::Block) {
        self.blocks.push(block);
    }
}
impl Hashing for Blockchain {
    fn encode_block(&self) -> Vec<u8> {
        let x = self.blocks.last();
        let serialized: Vec<u8> = bincode::serialize(&x).unwrap();
        return serialized;
    }
    fn hash(&self) -> [u8; 32] {
        // let mut hasher = Sha512::new();
        // hasher.input(&self.blocks);
        // let mut hash = [0; 32];
        // hasher.result(&mut hash);
        // return hash;
        return [0; 32]
    }
    fn get_last_block_hash(&self) -> [u8; 32] {
        return self.blocks.last().unwrap().header.prev_block_hash;
    }
}
