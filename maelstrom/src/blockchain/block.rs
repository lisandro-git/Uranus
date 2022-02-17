use super::*;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::io::ReadBuf;
use tokio::macros::support::poll_fn;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use std::io;
use std::net::SocketAddr;
use std::str::{EscapeDebug, from_utf8};
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub magic_number: u32,
    pub size: u32,
    pub header: self::Block_Header,
    pub data: self::Block_Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block_Data {
    bogus: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block_Header {
    pub version: [u8; 4],
    pub prev_block_hash: [u8; 32],
    pub timestamp: [u8; 8],
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

impl Block_Header {
    pub fn new(version: [u8; 4], prev_block_hash: [u8; 32], timestamp: [u8; 8]) -> Block_Header {
        Block_Header {
            version,
            prev_block_hash,
            timestamp,
        }
    }
}

impl Block_Data {
    pub fn new(d: u32) -> Block_Data {
        Block_Data {
            bogus: d
        }
    }
}
