#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused)]
extern crate core;

pub mod blockchain;
pub mod communication;
mod morse;

fn main() {
    //server::encryption::decrypt_message(vec![0]);
    communication::server::main();
    println!("Hello, world!");
}