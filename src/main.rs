use std::net::TcpListener;
use std::io::Write;
use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1").unwrap();
    println!("Hello, world!");
}
