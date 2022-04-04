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
use serde::{Deserialize, Serialize};
use serde_bytes::{deserialize, serialize};

use super::data_processing as dp;
use super::bot;

const LOCAL: &str = "127.0.0.1:6969";
const MSG_SIZE: usize = 4096;

async fn client_input (mut s_write: OwnedWriteHalf, mut B: bot::Bot) -> OwnedWriteHalf {
    loop {
        println!("-> ");
        let mut buff = String::new();
        stdin()
            .read_line(&mut buff)
            .expect("Did not entered a correct string");
        buff.pop();
        B.com.command = buff.clone().into_bytes();
        let x = dp::serialize_data(&B);
        println!("serialized: {:?}", x);
        let mut buffer = dp::obfuscate_data(x);
        println!("obfuscated data : {}, {:?}", buffer.len(), buffer);
        s_write.write_all(&buffer).await.unwrap();
    }
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    // TCP Stream creation
    let mut server =  TcpStream::connect(LOCAL).await?;
    let (mut reader, mut writer) = server.into_split();

    println!("Connecting to server...");
    let mut B = bot::Bot::new(Vec::new(), Vec::new());

    tokio::spawn(async move {
        client_input(writer, B).await;
    });
    loop {
        let mut buf = [0; MSG_SIZE];
        let data = reader.read(&mut buf[..]).await?;
        println!("Received data: {:?}", data);
    }
    Ok(())
}
