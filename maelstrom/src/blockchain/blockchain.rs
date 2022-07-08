use std::ops::{Deref, DerefMut};
use std::str::from_utf8;
use super::{
    block::{Block, Hashing},
    database
};
use leveldb::database as leveldb_database;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn new(mut genesis_block: Block) -> Blockchain {
        genesis_block.header.prev_block_hash = from_utf8(&[0u8; 32]).unwrap().to_string();
        database::write_db(&database::D, genesis_block.header.block_id.clone() as i32, genesis_block.serialize_block().as_slice());
        return Blockchain {
            blocks: vec![genesis_block],
        };
    }
    pub fn add_block(&mut self, mut Blk: Block) {
        // edode : calculating the previous Block's hash
        Blk.header.prev_block_hash = Blk.calculate_hash(&self.blocks.last().unwrap());
        database::write_db(&database::D, Blk.header.block_id.clone() as i32, Blk.serialize_block().as_slice());
        println!("Block : {:?} added to the blockchain", Blk.header.block_id);
        self.blocks.push(Blk);
        self.remove(0);
    }
    pub fn get_last_block_hash(&self) -> String {
        return self.blocks.last().unwrap().clone().header.prev_block_hash;
    }
}
impl Deref for Blockchain {
    type Target = Vec<Block>;
    fn deref(&self) -> &Self::Target {
        return &self.blocks;
    }
}
impl DerefMut for Blockchain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.blocks;
    }
}
