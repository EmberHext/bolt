mod utils;

pub fn launch_ws_service(port: u16, address: String) {
    println!("Starting WS service on {} port {}", address, port);

    std::thread::park();
}
