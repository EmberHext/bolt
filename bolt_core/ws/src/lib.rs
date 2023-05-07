mod utils;

use utils::*;

pub async fn launch_server(port: u16, address: String) {
    println!("Starting Websocket server on {} port {}", address, port);
}
