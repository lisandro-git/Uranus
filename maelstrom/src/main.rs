#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused)]
extern crate core;

use std::thread;
use tokio;
pub mod blockchain;
pub mod communication;
mod morse;

fn main() {
    //server::encryptor::decrypt_message(vec![0]);
    communication::server::main();
    //communication::client::main();
    println!("Hello, world!");
}