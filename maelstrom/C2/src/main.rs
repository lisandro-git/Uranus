#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused)]
extern crate core;

pub mod encoder;
pub mod encryption;
pub mod message;
pub mod blockchain;
pub mod communication;

fn main() {
    //server::encryptor::decrypt_message(vec![0]);
    communication::server::main();
    //communication::client::main();
    println!("Hello, world!");
}