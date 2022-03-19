extern crate core;

pub mod blockchain;
pub mod server;
mod morse;

/*
encryptions step :
    - data
    - base64
    - rsa

 */

fn main() {
    //server::encryption::decrypt_message(vec![0]);
    server::server::main();
    println!("Hello, world!");
}