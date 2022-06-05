use tokio::{
    io::{
        AsyncReadExt,
        BufReader,
        ReadBuf,
        AsyncWriteExt
    },
    macros::support::poll_fn,
    net::{TcpListener, TcpStream},
    sync::{
        broadcast,
        broadcast::Receiver,
        broadcast::Sender
    }
};
use std::{
    io,
    net::SocketAddr,
    str::{EscapeDebug, from_utf8}
};
use serde::{Deserialize, Serialize};
use crate::communication::bot;
use crate::communication::bot::{Bot, Device_stream};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block_Header {
    pub version: [u8; 4],
    pub prev_block_hash: [u8; 32],
    pub block_id: u64,
    pub timestamp: [u8; 8],
}
impl Block_Header {
    pub fn new(version: [u8; 4], prev_block_hash: [u8; 32], timestamp: [u8; 8], block_id: u64) -> Block_Header {
        Block_Header {
            version,
            prev_block_hash,
            block_id,
            timestamp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub magic_number: u32,
    pub size: u32,
    pub header: self::Block_Header,
    pub data: self::Block_Data,
}
impl Block {
    pub fn new(size: u32, BH: Block_Header, BD: Block_Data) -> Block {
        Block {
            magic_number: 0xF14ED0DE,
            size: 0, // edode : size of the entire block
            header: BH,
            data: BD,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block_Data {
    data: bot::Device_stream,
}
impl Block_Data {
    pub fn new(data: bot::Device_stream) -> Block_Data {
        Block_Data {
            data: data,
        }
    }
}
