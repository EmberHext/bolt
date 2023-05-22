use std::net::UdpSocket;
use std::thread;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:4444").expect("Failed to bind socket");

    println!("Server listening on {}", socket.local_addr().unwrap());

    let socket2 = socket.try_clone().unwrap();
    thread::spawn(move || {
        let mut buf = [0; 1024];
        loop {
            match socket2.recv_from(&mut buf) {
                Ok((received_bytes, _)) => {
                    let received_data = &buf[..received_bytes];
                    println!("Received: {:?}", received_data);
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                }
            }
        }
    });

    loop {
        socket
            .send_to("Heyyyyyyyy".as_bytes(), "127.0.0.1:4445")
            .expect("Failed to send data");

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
