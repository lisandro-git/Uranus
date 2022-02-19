pub mod blockchain;
pub mod server;

fn main() {
    server::server::main();
    println!("Hello, world!");
}