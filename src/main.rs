use std::net::{ TcpListener };
use std::io::Write;
use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening started, ready to accept!");
    for data in listener.incoming(){
        thread::spawn(|| {
            let mut data = data.unwrap();
            data.write(b"Hello, World!\r\n").unwrap();
        });

    }

}
