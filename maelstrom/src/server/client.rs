use tokio::{
    net::TcpStream,
    io::{AsyncBufReadExt, AsyncReadExt},
    io::AsyncWriteExt,
    io,
    net::tcp::OwnedWriteHalf
};
use futures::lock::Mutex;
use std::{
    sync::Arc,
    io::stdin,
    net::SocketAddr,
    str::from_utf8
};
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const IV_LEN: usize = 16;
const USERNAME_LENGTH: usize = 10;

struct Client {
    authenticated: bool,
    connected: bool,
    IM: Incoming_Message,
    OM: Outgoing_Message,
}
impl Client {
    fn new() -> Client {
        Client {
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
    fn clone(&self) -> Outgoing_Message {
        Outgoing_Message {
            username: self.username.clone(),
            data: self.data.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
    fn clone(&self) -> Incoming_Message {
        Incoming_Message {
            username: self.username.clone(),
            data: self.data.clone(),
        }
    }
}

fn remove_trailing_zeros(data: Vec<u8>) -> Vec<u8> {
    // Used to remove the zeros at the end of the received encrypted message
    // but not inside the message (purpose of the 'keep_push' var

    let mut transit:Vec<u8> = vec![];
    let mut res:Vec<u8> = vec![];
    let mut keep_push = false;
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
    return res.to_owned();
}

fn create_username() -> Vec<u8> {
    let mut username = String::with_capacity(1);
    loop {
        println!("Username : ");
        std::io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
        if username.len() > 1 && username.len() <= USERNAME_LENGTH {
            break;
        } else {
            println!("Username must be between 1 and 10 characters");
        }
    }
    username.pop();
    return truncate_in_place(username, USERNAME_LENGTH).to_owned();
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn truncate_in_place(mut s: String, max_chars: usize) -> Vec<u8>{
    let bytes = truncate(&s, max_chars).len();
    s.truncate(bytes);
    let mut buffer = [0u8; USERNAME_LENGTH];
    buffer[.. s.len()].copy_from_slice(s.as_bytes());
    return remove_trailing_zeros(buffer.to_vec());
}

async fn client_input (mut s_write: OwnedWriteHalf, mut OM: Arc<Mutex<Outgoing_Message>>) -> OwnedWriteHalf {
    loop {
        println!("Enter message : ");
        let mut buff = String::new();
        stdin()
            .read_line(&mut buff)
            .expect("Did not entered a correct string");
        buff.pop();

        let mut M_lock = OM.lock().await;

        M_lock.data = buff.trim().as_bytes().to_vec();
        let serialized_data = serialize(&M_lock.clone()).unwrap();
        M_lock.data.clear();
        drop(M_lock);
        println!("Sending message : {:?}", serialized_data);
        s_write.write_all(&serialized_data).await.unwrap();
    }
}

fn serialize_data(M: &Outgoing_Message) -> Vec<u8>{
    return serialize(&M).unwrap();
}

fn deserialize_data(data: Vec<u8>) -> Incoming_Message {
    return deserialize(&data).unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Username input

    // TCP Stream creation
    let mut server =  TcpStream::connect(LOCAL).await?;
    let (mut reader, mut writer) = server.into_split();

    println!("Connecting to server...");
    let mut C = Client::new();
    let username = create_username();
    C.OM.username = username.clone();
    let serialized_data = serialize_data(&C.OM);
    println!("Sent data: {:?}", serialized_data);
    writer.write_all(serialized_data.as_slice()).await.unwrap();


    let OM_mut = Arc::new(Mutex::new(C.OM));
    tokio::spawn(async move {
        client_input(writer, OM_mut).await;
    });
    loop {
        let mut buf = [0; MSG_SIZE];
        let data = reader.read(&mut buf[..]).await?;
        println!("Received data: {:?} jdlksqjdsq", data);
    }
    Ok(())
}
