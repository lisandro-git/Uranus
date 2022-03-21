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
use crate::server::encryption::decrypt_message;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const USERNAME_LENGTH: usize = 10;
const IV_LEN: usize = 16;
const RFC4648_ALPHABET: &'static [u8]   = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

#[derive(Debug)]
pub struct Bot {
    stream: TcpStream,
    ip_address: std::net::SocketAddr,
    authenticated: bool,
    connected: bool,
    M: Message,
}
impl Bot {
    fn new(sock: TcpStream, address: SocketAddr) -> Bot {
        Bot {
            stream: sock,
            ip_address: address,
            authenticated: false,
            connected: true,
            M: Message::new(),
        }
    }
    fn erase_data(&mut self) {
        //self.sock.shutdown(Shutdown::Both).unwrap();
        self.M.erase_data();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(with = "serde_bytes")]
    Username: Vec<u8>,
    #[serde(with = "serde_bytes")]
    Data: Vec<u8>,
    #[serde(with = "serde_bytes")]
    Command: Vec<u8>,
}
impl Message {
    fn new() -> Message {
        Message { Username: vec![], Data: vec![], Command: vec![] }
    }
    fn erase_data(&mut self) {
        //self.Username.clear();
        self.Data.clear();
        self.Command.clear();
    }
}

fn remove_trailing_zeros(data: Vec<u8>) -> Vec<u8> {
    // Used to remove the zeros at the end of the received encrypted message
    // but not inside the message (purpose of the 'keep_push' var

    let mut transit: Vec<u8> = vec![];
    let mut res: Vec<u8> = vec![];
    let mut keep_push: bool = false;
    for d in data.iter().rev() {
        if *d == 0 && !keep_push{
            continue;
        } else {
            transit.push(*d);
            keep_push = true;
        }
    }
    for t in transit.iter().rev() {
        res.push(*t);
    }
    //res.push(0)
    return res.to_owned();
}

async fn handle_message_received(B: &mut Bot) -> Vec<u8> {
    let mut buffer = [0; MSG_SIZE];
    loop {
        B.stream.readable().await;
        match B.stream.try_read(&mut buffer) {
            Ok(0) => {
                //println!("Client {} (username : {:?}) disconnected", B.ip_address, from_utf8(&B.M.Username).unwrap());
                B.connected = false;
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

fn serialize_data(M: &Message) -> Vec<u8>{
    return rmp_serde::to_vec_named(&M).unwrap();
}

fn deserialize_message(data: Vec<u8>) -> Message {
    return rmp_serde::from_read(data.as_slice()).unwrap();
}

fn deobfuscate_data(morse_code: Vec<u8>) -> Vec<u8> {
    let base32_encoding = base32::Alphabet::RFC4648 { padding: true };
    let base32_data = morse::morse_to_word::decode(remove_trailing_zeros(morse_code));
    let encrypted_data = base32::decode(
        base32_encoding,
        from_utf8(base32_data.as_slice()).unwrap()).unwrap();
    return decrypt_message(encrypted_data.to_vec());
}

async fn authenticate_new_user(socket: TcpStream, addr: SocketAddr) -> Bot {
    let mut B = Bot {
        stream: socket,
        ip_address: addr,
        authenticated: false,
        connected: true,
        M: Message::new(),
    };
    let data = handle_message_received(&mut B).await;
    if B.connected {
        let clear_data = deobfuscate_data(data);
        B.M = deserialize_message(clear_data);

    }
    println!("Authenticating new user : {:?}", B.M.Username);
    return B;
}

async fn handle_message_from_client(mut B: Bot, channel_snd: Sender<Message>, mut channel_rcv: Receiver<Message>) -> Bot {
    let mut buffer: [u8; 4096] = [0; MSG_SIZE];

    loop{
        match B.stream.try_read(&mut buffer) {
            Ok(n) if n == 0 => {
                println!("Client {} (username : {:?}) disconnected", B.ip_address, from_utf8(&B.M.Username).unwrap());
                B.connected = false;
                return B;
            },
            Ok(recv_bytes) => {
                println!("Received bytes: {}", recv_bytes);
                // decoding the base64 before doing this
                
                let decrypted_data = encryption::decrypt_message(remove_trailing_zeros(buffer.to_vec()));
                let encrypted_data = remove_trailing_zeros(decrypted_data);
                B.M = deserialize_message(encrypted_data);
                channel_snd.send(B.M.clone()).unwrap();
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
                println!("Received data from channel : {:?}, from : {:?}", received_data, B.ip_address);
                //sending the data to other users
                println!("Sending data to {}", B.ip_address);

                B.stream.write(&serialize_data(&received_data)).await.unwrap();
                B.M.erase_data();
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
        let mut B: Bot = authenticate_new_user(socket, addr).await;
        if !B.connected {
            drop(B);
            continue;
        }
        // Thread creation
        let thread_send = channel_snd.clone();
        let thread_rcv = channel_snd.subscribe();

        tokio::spawn(async move {
            handle_message_from_client(B, thread_send, thread_rcv).await;
        });
    }
}
