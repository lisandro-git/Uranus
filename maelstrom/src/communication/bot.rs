extern crate core;
use tokio::{
    net::{TcpListener, TcpStream},
};
use std::{
    net::SocketAddr,
};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Debug)]
pub struct Device_stream {
    pub stream: TcpStream,
    pub ip_address: std::net::SocketAddr,
    pub authenticated: bool,
    pub connected: bool,
    pub B: Bot,
}
impl Device_stream {
    pub fn new(sock: TcpStream, address: SocketAddr, uid: Vec<u8>, version: Vec<u8>) -> Device_stream {
        Device_stream {
            stream: sock,
            ip_address: address,
            authenticated: false,
            connected: true,
            B: Bot::new(uid, version),
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
    pub com: Commands
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