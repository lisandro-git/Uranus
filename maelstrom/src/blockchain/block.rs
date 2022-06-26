use std::{
    io,
    net::SocketAddr,
};
use serde::{Deserialize, Serialize};
use bincode;
use sha2::{
    Sha512,
    Digest,
    digest::Update
};
use deepsize::DeepSizeOf;
use crate::{
    communication::c2,
    communication::c2::{Bot, Device_stream},
    blockchain::blockchain::Hashing
};
use super::blockchain::Blockchain;

pub trait Genesis {
    fn genesis_block() -> Self;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub magic_number: u32,
    pub size: u32,
    pub header: BlockHeader,
    pub data: BlockData,
}
impl Block {
    pub fn new() -> Block {
        let mut B = Block {
            magic_number: 0xF14ED0DE,
            size: 0, // edode : size of the entire block
            header: self::BlockHeader::new(),
            data: self::BlockData::new(),
        };
        B.size = self::Block::get_block_size(&B);
        return B;
    }
    pub fn update_block(&mut self, BH: BlockHeader, BD: BlockData) {
        self.header = BH;
        self.data = BD;
        self.size = self.get_block_size();
    }
    fn get_block_size(&self) -> u32 {
        return (self.header.deep_size_of() as u32) + (self.data.deep_size_of() as u32);
    }
}
impl Genesis for Block { // lisandro : can be merged with new()
    fn genesis_block() -> Self {
        return Block::new();
    }
}

#[derive(Debug, Serialize, Deserialize, DeepSizeOf, Clone)]
pub struct BlockHeader {
    pub version: [u8; 4],
    pub prev_block_hash: [u8; 32],
    pub block_id: u64,
    pub timestamp: [u8; 8],
}
impl BlockHeader {
    pub fn new() -> BlockHeader {
        return BlockHeader {
            version: [0; 4],
            prev_block_hash: [0; 32],
            block_id: 0,
            timestamp: [0; 8],
        }
    }
    pub fn create_block_header(&mut self, last_block_hash: [u8; 32]) {
        self.version = [0; 4];
        self.prev_block_hash = last_block_hash;
        self.block_id += 1;
        self.timestamp = self::BlockHeader::update_timestamp();
    }
    fn update_timestamp() -> [u8; 8] {
        return [0, 0, 0, 0, 0, 0, 0, 0];
    }
}

#[derive(Debug, Serialize, Deserialize, DeepSizeOf, Clone)]
pub struct BlockData {
    bot: c2::Device_stream,
}
impl BlockData {
    pub fn new() -> BlockData {
        return BlockData {
            bot: c2::Device_stream::new(
                String::new(),
                false,
                false,
                Vec::new(),
                c2::Bot::new(Vec::new(), Vec::new())
            ),
        }
    }
    pub fn create_block_data(&mut self, DS: c2::Device_stream) {
        self.bot = DS;
    }
}
