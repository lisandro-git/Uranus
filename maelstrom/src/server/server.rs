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
use crate::server::encryption;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const USERNAME_LENGTH: usize = 10;
const IV_LEN: usize = 16;

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    ip_address: std::net::SocketAddr,
    authenticated: bool,
    connected: bool,
    M: Message,
}
impl Client {
    fn new(sock: TcpStream, address: SocketAddr) -> Client {
        Client {
            stream: sock,
            ip_address: address,
            authenticated: false,
            connected: true,
            M: Message::new(vec![], vec![], vec![]),
        }
    }
    fn clear(&mut self) {
        //self.sock.shutdown(Shutdown::Both).unwrap();
        self.M.clear();
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
    fn new(Username: Vec<u8>, Data: Vec<u8>, Command: Vec<u8>) -> Message {
        Message { Username, Data, Command}
    }
    fn clear(&mut self) {
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
    //res.push(0);
    return res.to_owned();
}

async fn handle_message_received(C: &mut Client) -> Vec<u8> {
    let mut buffer = [0; MSG_SIZE];
    loop {
        C.stream.readable().await;
        match C.stream.try_read(&mut buffer) {
            Ok(0) => {
                //println!("Client {} (username : {:?}) disconnected", C.ip_address, from_utf8(&C.M.Username).unwrap());
                C.connected = false;
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

fn serialize_data(OM: &Message) -> Vec<u8>{
    return rmp_serde::to_vec_named(&OM).unwrap();
}

fn deserialize_message(data: Vec<u8>) -> Message {
    return rmp_serde::from_read(data.as_slice()).unwrap();
}

async fn authenticate_new_user(socket: TcpStream, addr: SocketAddr) -> Client {
    let mut C = Client {
        stream: socket,
        ip_address: addr,
        authenticated: false,
        connected: true,
        M: Message::new(vec![], vec![], vec![]),
    };
    let data = handle_message_received(&mut C).await;
    if C.connected {
        let decrypted_data =  encryption::decrypt_message(remove_trailing_zeros(data));
        let encrypted_data = remove_trailing_zeros(decrypted_data);
        C.M = deserialize_message(encrypted_data);
    }
    println!("Authenticating new user : {:?}", C.M.Username);
    return C;
}

// send to everyone except the sender
async fn send_to_all_except(C: &mut Client, msg: Vec<u8>) {
    let mut clients = CLIENT_LIST.lock().unwrap();
    for client in clients.iter_mut() {
        if client.ip_address != C.ip_address {
            client.stream.write_all(&msg).await.unwrap();
        }
    }
}

async fn handle_message_from_client(mut C: Client, channel_snd: Sender<Message>, mut channel_rcv: Receiver<Message>, ) -> Client {
    let mut buffer: [u8; 4096] = [0; MSG_SIZE];

    loop{
        match C.stream.try_read(&mut buffer) {
            Ok(n) if n == 0 => {
                println!("Client {} (username : {:?}) disconnected", C.ip_address, from_utf8(&C.M.Username).unwrap());
                C.connected = false;
                return C;
            },
            Ok(recv_bytes) => {
                println!("Received bytes: {}", recv_bytes);
                let decrypted_data =  encryption::decrypt_message(remove_trailing_zeros(buffer.to_vec()));
                let encrypted_data = remove_trailing_zeros(decrypted_data);
                C.M = deserialize_message(encrypted_data);
                channel_snd.send(C.M.clone()).unwrap();
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
                println!("Received data from channel : {:?}, from : {:?}", received_data, C.ip_address);
                //sending the data to other users
                println!("Sending data to {}", C.ip_address);

                C.stream.write(&serialize_data(&received_data)).await.unwrap();
                C.M.clear();
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
        let mut C: Client = authenticate_new_user(socket, addr).await;
        if !C.connected {
            drop(C);
            continue;
        }
        // Thread creation
        let thread_send = channel_snd.clone();
        let thread_rcv = channel_snd.subscribe();

        tokio::spawn(async move {
            handle_message_from_client(C, thread_send, thread_rcv).await;
        });
    }
}
