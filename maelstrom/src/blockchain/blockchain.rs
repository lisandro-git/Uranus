use std::ops::{Deref, DerefMut};
use std::str::from_utf8;
use leveldb::database as leveldb_database;
use super::{
    block::{Block, Hashing},
    database
};

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn new(mut genesis_block: Block) -> Blockchain {
        genesis_block.header.prev_block_hash = from_utf8(&[0u8; 32]).unwrap().to_string();
        database::write_db(&database::D, genesis_block.header.block_id.clone() as i32, genesis_block.serialize_block().as_slice());
        println!("Block : {:?} added to the blockchain", genesis_block.header.block_id);
        return Blockchain {
            blocks: vec![genesis_block],
        };
    }
    pub fn add_block(&mut self, mut Blk: Block) {
        Blk.header.prev_block_hash = Blk.calculate_hash(database::get_last_db_value(&database::D).as_slice());
        database::write_db(&database::D, Blk.header.block_id.clone() as i32, Blk.serialize_block().as_slice());
        println!("Block N°{:?} added to the blockchain", Blk.header.block_id);
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
