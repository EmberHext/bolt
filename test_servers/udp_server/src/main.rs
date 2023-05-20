use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:4444").expect("Failed to bind socket");

    println!("Server listening on {}", socket.local_addr().unwrap());

    loop {
        socket
            .send_to("Heyyyyyyyy".as_bytes(), "127.0.0.1:4445")
            .expect("Failed to send data");
   
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
