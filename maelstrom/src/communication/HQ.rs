use tokio::{
    net::TcpStream,
    io::{AsyncBufReadExt, AsyncReadExt},
    io::AsyncWriteExt,
    io,
    net::tcp::OwnedWriteHalf,
    net::TcpListener,
};
use futures::lock::Mutex;
use std::{
    sync::Arc,
    io::stdin,
    net::SocketAddr,
    str::from_utf8
};
use serde::{Deserialize, Serialize};
use serde_bytes::{deserialize, serialize};

use super::data_processing as dp;
use super::bot;

const LOCAL: &str = "127.0.0.1:6969";
const MSG_SIZE: usize = 4096;

#[tokio::main]
pub async fn main() -> io::Result<()> {

    let listener = TcpListener::bind(LOCAL).await?;

    //let (bot_input_sender, bot_input_receiver) = broadcast::channel(64);
    println!("HQ Server Initialized");
    loop {
        // User accept
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New user connected: {}", addr);


        //tokio::spawn(async move {
        //    client_input(tr, &mut c_cmd).await;
        //});

    }
}
