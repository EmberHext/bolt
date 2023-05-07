mod utils;

use utils::*;

pub async fn launch_server(port: u16, address: String) {
    println!("Starting UDP server on {} port {}", address, port);
}
