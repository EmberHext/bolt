use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    Message,
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    println!("started on ws://0.0.0.0:3012");

    let server = TcpListener::bind("0.0.0.0:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |_req: &Request, response: Response| {
                println!("new connection");

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    handle_msg(msg.clone());
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}

fn handle_msg(msg: Message) {
    let txt = match msg {
        Message::Text(txt) => txt,
        _ => panic!("bytes")
    };

    println!("MSG: {}", txt);
}
