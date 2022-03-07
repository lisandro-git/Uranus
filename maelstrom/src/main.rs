extern crate core;

pub mod blockchain;
pub mod server;

fn main() {
    //server::encryption::decrypt_message(vec![0]);
    server::server::main();
    println!("Hello, world!");
}