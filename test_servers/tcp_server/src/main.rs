use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 128];

    while stream.read(&mut buf).unwrap() != 0 {
        stream.write(&buf).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5555").unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        std::thread::spawn(|| handle_client(stream.unwrap()));
    }
}
