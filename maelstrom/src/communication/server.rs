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

use crate::communication::encryption;
use crate::morse;
use crate::communication::lib;
use crate::communication::data_processing as dp;
use super::bot;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const USERNAME_LENGTH: usize = 10;
const IV_LEN: usize = 16;

async fn handle_message_received(DS: &mut bot::Device_stream) -> Vec<u8> {
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

async fn authenticate_new_user(socket: TcpStream, addr: SocketAddr) -> bot::Device_stream {
    let mut DS = bot::Device_stream {
        stream: socket,
        ip_address: addr,
        authenticated: false,
        connected: true,
        B: bot::Bot::new(Vec::new(), Vec::new()),
    };
    let data = handle_message_received(&mut DS).await;
    if DS.connected {
        let clear_data = dp::deobfuscate_data(data);
        DS.B = dp::deserialize_message(clear_data);
    }
    println!("Authenticating new user : {:?}", DS.B.uid);
    return DS;
}

async fn handle_message_from_client(mut DS: bot::Device_stream, channel_snd: Sender<bot::Bot>, mut channel_rcv: Receiver<bot::Bot>) -> bot::Device_stream {
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
        let mut DS: bot::Device_stream = authenticate_new_user(socket, addr).await;
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
