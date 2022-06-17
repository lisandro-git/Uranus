#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused)]
extern crate core;

use std::thread;

pub mod encoder;
pub mod encryption;
pub mod message;
pub mod communication;
pub mod blockchain;

fn main() {
    //communication::sandbox::main();
    communication::server::main();
}