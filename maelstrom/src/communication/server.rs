
extern crate core;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, Interest, ReadBuf},
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
    slice,
    net::SocketAddr,
    process::Command,
    ptr::slice_from_raw_parts,
    str::{EscapeDebug, from_utf8}
};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use base32;

use crate::server::encryption;
use crate::morse;
use crate::server::lib;
use crate::server::data_processing as dp;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const USERNAME_LENGTH: usize = 10;
const IV_LEN: usize = 16;



#[derive(Debug)]
pub struct Device_stream {
    stream: TcpStream,
    ip_address: std::net::SocketAddr,
    authenticated: bool,
    connected: bool,
    B: Bot,
}
impl Device_stream {
    fn new(sock: TcpStream, address: SocketAddr, uid: Vec<u8>, version: Vec<u8>) -> Device_stream {
        Device_stream {
            stream: sock,
            ip_address: address,
            authenticated: false,
            connected: true,
            B: Bot::new(uid, version),
        }
    }
    fn erase_data(&mut self) {
        //self.sock.shutdown(Shutdown::Both).unwrap();
        self.B.erase_data();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bot {
    #[serde(with = "serde_bytes")]
    uid: Vec<u8>,
    #[serde(with = "serde_bytes")]
    version: Vec<u8>,
    com: Commands
}
impl Bot {
    fn new(uid: Vec<u8>, version: Vec<u8>) -> Bot {
        Bot {
            uid: uid,
            version: version,
            com: Commands::new(Vec::new(), Vec::new())
        }
    }
    fn erase_data(&mut self) {
        self.com.erase_data();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commands {
    #[serde(with = "serde_bytes")]
    command: Vec<u8>,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}
impl Commands {
    fn new(command: Vec<u8>, data: Vec<u8>) -> Commands {
        Commands {
            command: command,
            data: data,
        }
    }
    fn erase_data(&mut self) {
        self.command.clear();
        self.data.clear();
    }
}

async fn handle_message_received(DS: &mut Device_stream) -> Vec<u8> {
    let mut buffer = [0; MSG_SIZE];
    loop {
        DS.stream.readable().await;
        match DS.stream.try_read(&mut buffer) {
            Ok(0) => {
                //println!("Client {} (username : {:?}) disconnected", DS.ip_address, from_utf8(&DS.M.Username).unwrap());
                DS.connected = false;
                return vec![];
            }
            Ok(recv_bytes) => {
                println!("Received bytes: {}", recv_bytes);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // edode : Avoid returning an empty vector (empty Incoming_Message)
                println!("Error: {}", e);
                continue;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
        println!("buffer read : {:?}", buffer);
        return buffer.to_vec();
    };
}

async fn authenticate_new_user(socket: TcpStream, addr: SocketAddr) -> Device_stream {
    let mut DS = Device_stream {
        stream: socket,
        ip_address: addr,
        authenticated: false,
        connected: true,
        B: Bot::new(Vec::new(), Vec::new()),
    };
    let data = handle_message_received(&mut DS).await;
    if DS.connected {
        let clear_data = dp::deobfuscate_data(data);
        DS.B = dp::deserialize_message(clear_data);
    }
    println!("Authenticating new user : {:?}", DS.B.uid);
    return DS;
}

async fn handle_message_from_client(mut DS: Device_stream, channel_snd: Sender<Bot>, mut channel_rcv: Receiver<Bot>) -> Device_stream {
    let mut buffer: [u8; 4096] = [0; MSG_SIZE];

    loop{
        match DS.stream.try_read(&mut buffer) {
            Ok(n) if n == 0 => {
                println!("Client {} (username : {:?}) disconnected", DS.ip_address, from_utf8(&DS.B.uid).unwrap());
                DS.connected = false;
                return DS;
            },
            Ok(recv_bytes) => {
                println!("Received bytes: {}", recv_bytes);
                let clear_data = dp::deobfuscate_data(buffer.to_vec());
                DS.B = dp::deserialize_message(clear_data);
                channel_snd.send(DS.B.clone()).unwrap();
                buffer.iter_mut().for_each(|x| *x = 0); // reset buffer
            },
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                // edode : Avoid returning an empty vector (empty Incoming_Message)
                //println!("Error: {}", err);
                continue;
            },
            Err(err) => {
                println!("Error: {}", err);
            },
        };

        match channel_rcv.try_recv() {
            Ok(mut received_data) => {
                println!("Received data from channel : {:?}, from : {:?}", received_data, DS.ip_address);
                //sending the data to other users
                println!("Sending data to {}", DS.ip_address);

                DS.stream.write(&dp::serialize_data(&received_data)).await.unwrap();
                DS.B.erase_data();
            },
            Err(err) => {
                // edode : empty channel
                //println!("Could not receive data : {}", err);
            },
        };
    }
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(LOCAL).await?;
    let (channel_snd, mut _chann_rcv)  = broadcast::channel(64);
    println!("Server Initialized");
    loop {
        // User accept
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("New user connected: {}", addr);
        let mut DS: Device_stream = authenticate_new_user(socket, addr).await;
        if !DS.connected {
            drop(DS);
            continue;
        }
        // Thread creation
        let thread_send = channel_snd.clone();
        let thread_rcv = channel_snd.subscribe();

        tokio::spawn(async move {
            handle_message_from_client(DS, thread_send, thread_rcv).await;
        });
    }
}
