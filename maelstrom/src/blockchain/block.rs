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

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub magic_number: u32,
    pub size: u32,
    pub header: BlockHeader,
    pub data: BlockData,
}
impl Block {
    pub fn new(
        BH: BlockHeader,
        BD: BlockData
    ) -> Block {
        return Block {
            magic_number: 0xF14ED0DE,
            size: 0, // edode : size of the entire block
            header: BH,
            data: BD,
        }
    }
    pub fn create_block(&mut self, BD: BlockData, BH: BlockHeader) {
        self.header = BH;
        self.data = BD;
        self.size = self.get_block_size();
    }
    fn get_block_size(&self) -> u32 {
        return (self.header.deep_size_of() as u32) + (self.data.deep_size_of() as u32);
    }
}
impl Genesis for Block {
    fn genesis_block() -> Self {
        Block::new(
            self::BlockHeader::new(
                [0; 4],
                [0; 32],
                0,
                [0; 8],
            ),
            self::BlockData::new(
                Device_stream::new(
                    String::new(),
                    false,
                    false,
                    Vec::new(),
                    Bot::new(Vec::new(), Vec::new())
                )
            ),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, DeepSizeOf)]
pub struct BlockHeader {
    pub version: [u8; 4],
    pub prev_block_hash: [u8; 32],
    pub block_id: u64,
    pub timestamp: [u8; 8],
}
impl BlockHeader {
    pub fn new(
        version: [u8; 4],
        prev_block_hash: [u8; 32],
        block_id: u64,
        timestamp: [u8; 8],
    ) -> BlockHeader {
        return BlockHeader {
            version,
            prev_block_hash,
            block_id,
            timestamp,
        }
    }
    pub fn update_block_header(&mut self) {
        self.version;
        self.prev_block_hash = [0; 32];//Blockchain::get_last_block_hash();
        self.block_id += 1;
        self.timestamp = self.update_timestamp();
    }
    pub fn update_timestamp(&mut self) -> [u8; 8] {
        return [0, 0, 0, 0, 0, 0, 0, 0];
    }
}

#[derive(Debug, Serialize, Deserialize, DeepSizeOf)]
pub struct BlockData {
    data: c2::Device_stream,
}
impl BlockData {
    pub fn new(data: c2::Device_stream) -> BlockData {
        return BlockData {
            data,
        }
    }
}
