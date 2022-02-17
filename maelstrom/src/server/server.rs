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

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const IV_LEN: usize = 16;
const USERNAME_LENGTH: usize = 10;

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    ip_address: std::net::SocketAddr,
    authenticated: bool,
    connected: bool,
    IM: Incoming_Message,
    OM: Outgoing_Message,
}
impl Client {
    fn new(sock: TcpStream, address: SocketAddr) -> Client {
        Client {
            stream: sock,
            ip_address: address,
            authenticated: false,
            connected: true,
            IM: Incoming_Message::new(vec![], vec![]),
            OM: Outgoing_Message::new(vec![], vec![]),
        }
    }
    fn delete(&mut self) {
        //self.sock.shutdown(Shutdown::Both).unwrap();
        self.IM.delete();
        self.OM.delete();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Outgoing_Message {
    username: Vec<u8>,
    data: Vec<u8>,
}
impl Outgoing_Message {
    fn new(username: Vec<u8>, data: Vec<u8>) -> Outgoing_Message {
        Outgoing_Message { username, data }
    }
    fn delete(&mut self) {
        self.username.clear();
        self.data.clear();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Incoming_Message {
    username: Vec<u8>,
    data: Vec<u8>,
}
impl Incoming_Message {
    fn new(username: Vec<u8>, data: Vec<u8>) -> Incoming_Message {
        Incoming_Message { username, data }
    }
    fn delete(&mut self) {
        self.username.clear();
        self.data.clear();
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
                println!("Client {} (username : {:?}) disconnected", C.ip_address, from_utf8(&C.IM.username).unwrap());
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

fn serialize_data(OM: &Incoming_Message) -> Vec<u8>{
    return serialize(&OM).unwrap();
}

fn deserialize_message(data: Vec<u8>) -> Incoming_Message {
    return deserialize(&data).unwrap();
}

async fn authenticate_new_user(socket: TcpStream, addr: SocketAddr) -> Client {
    let mut C = Client {
        stream: socket,
        ip_address: addr,
        authenticated: false,
        connected: true,
        IM: Incoming_Message::new(vec![], vec![]),
        OM: Outgoing_Message::new(vec![], vec![]),
    };
    let username = handle_message_received(&mut C).await;
    if C.connected{
        C.IM = deserialize_message(username);
    }
    return C;
}

async fn handle_message_from_client(mut C: Client, channel_snd: Sender<Incoming_Message>, mut channel_rcv: Receiver<Incoming_Message>, ) -> Client{

    let mut buffer: [u8; 4096] = [0; MSG_SIZE];
    loop{
        //println!("loop");
        match channel_rcv.try_recv() {
            Ok(mut received_data) => {
                println!("Received data from channel : {:?}, from : {:?}", received_data, C.ip_address);
                //sending the data to other users
                println!("Sending data to {}", C.ip_address);
                C.stream.write(&serialize_data(&received_data)).await.unwrap();

            },
            Err(_) => {

            }
        }

        match C.stream.try_read(&mut buffer) {
            Ok(0) => {
                println!("Client {} (username : {:?}) disconnected", C.ip_address, from_utf8(&C.IM.username).unwrap());
                C.connected = false;
                return C;
            }
            Ok(recv_bytes) => {
                println!("Received bytes: {}", recv_bytes);
                C.IM.data = remove_trailing_zeros(buffer.to_vec());
                channel_snd.send(C.IM.clone()).unwrap();
                buffer.iter_mut().for_each(|x| *x = 0); // reset buffer
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // edode : Avoid returning an empty vector (empty Incoming_Message)
                //println!("Error: {}", e);
                //continue;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    }
}

fn broadcast_message(channel_snd: Sender<Incoming_Message>, OM: &Outgoing_Message) {
    let msg = Incoming_Message::new(OM.username.clone(), OM.data.clone());
    channel_snd.send(msg).unwrap();
}

#[tokio::main]
pub(crate) async fn main() -> io::Result<()> {
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
