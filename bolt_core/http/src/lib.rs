mod utils;

use utils::*;

pub async fn launch_server(port: u16, address: String) {
    println!("Starting TCP server on {} port {}", address, port);
}
