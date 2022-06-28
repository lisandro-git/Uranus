use super::blockchain;
use crate::communication::c2;
use std::{
    io,
    net::SocketAddr,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ptr::hash,
    time::SystemTime
};
use hex;
use bincode;
use deepsize::DeepSizeOf;
use sha3::{Digest, Sha3_512};
use serde::{Deserialize, Serialize};

const VERSION: &str = "0.1.0";

pub trait Genesis {
    fn genesis_block() -> Self;
}

pub trait Hashing {
    fn serialize_block(&self) -> Vec<u8>;
    fn calculate_hash<T: Hash>(&self, t: &T) -> String;
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
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
impl Hashing for Block {
    fn serialize_block(&self) -> Vec<u8> {
        //use bincode to serialize the block
        return bincode::serialize(&self).unwrap();
    }
    fn calculate_hash<T: Hash>(&self, previous_block: &T) -> String {
        let serialized_block = self.serialize_block();
        let mut hasher = Sha3_512::new();
        hasher.update(&serialized_block);
        let result = hasher.finalize();
        let hashed_block = hex::encode(&result[..]);

        let mut hasher = Sha3_512::new();
        hasher.update(&hashed_block);
        let hashed_hash = hasher.finalize();
        return hex::encode(&hashed_hash[..]);
    }
}

#[derive(Debug, Serialize, Deserialize, DeepSizeOf, Clone, Hash)]
pub struct BlockHeader {
    pub version: String,
    pub prev_block_hash: String,
    pub block_id: u64,
    pub timestamp: u64,
}
impl BlockHeader {
    pub fn new() -> BlockHeader {
        return BlockHeader {
            version: BlockHeader::get_version(),
            prev_block_hash: String::new(),
            block_id: 0,
            timestamp: self::BlockHeader::update_timestamp(),
        }
    }
    pub fn create_block_header(&mut self, last_block_hash: String) {
        self.version = self::BlockHeader::get_version();
        self.prev_block_hash = last_block_hash;
        self.block_id += 1;
        self.timestamp = self::BlockHeader::update_timestamp();
    }
    fn update_timestamp() -> u64 {
        return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    }
    fn get_version() -> String {
        return VERSION.to_string();
    }
}

#[derive(Debug, Serialize, Deserialize, DeepSizeOf, Clone, Hash)]
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
