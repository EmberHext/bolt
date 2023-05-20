mod utils;

use bolt_common::prelude::*;
use std::sync::{Arc, Mutex};
// use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
// use url::Url;
use std::net::UdpSocket;

const UDP_SERVICE_REFRESH_RATE: u64 = 500;
const SERVICE_SYNC_REFRESH_RATE: u64 = 1000;

// Create a shared global state variable
lazy_static::lazy_static! {
 static ref CORE_STATE: Arc<Mutex<CoreState>> = Arc::new(Mutex::new(CoreState::new()));
}

#[derive(Clone)]
struct UdpService {
    connection_id: String,
}

pub struct CoreState {
    main_state: MainState,
    session_websocket: Option<WebSocket<std::net::TcpStream>>,
    udp_services: Vec<UdpService>,
}

impl CoreState {
    pub fn new() -> Self {
        Self {
            main_state: MainState::new(),
            session_websocket: None,
            udp_services: vec![],
        }
    }
}

pub fn set_session_websocket(new_ws: WebSocket<std::net::TcpStream>) {
    let mut core_state = CORE_STATE.lock().unwrap();
    core_state.session_websocket = Some(new_ws);
}

pub fn set_main_state(client_state: MainState) {
    let mut core_state = CORE_STATE.lock().unwrap();
    core_state.main_state = client_state;
}

pub fn start_core_udp_service(_session_id: String) {
    std::thread::spawn(|| {
        // comment

        loop {
            let mut core_state = CORE_STATE.lock().unwrap();

            let connections = core_state.main_state.udp_connections.clone();
            let udp_services = core_state.udp_services.clone();

            for udp_con in connections.clone() {
                let exists = udp_services
                    .iter()
                    .any(|x| x.connection_id == udp_con.connection_id);

                if !exists {
                    spawn_udp_service(udp_con.connection_id.clone());

                    core_state.udp_services.push(UdpService {
                        connection_id: udp_con.connection_id,
                    });
                }
            }

            drop(core_state);
            std::thread::sleep(std::time::Duration::from_millis(SERVICE_SYNC_REFRESH_RATE));
        }
    });
}

pub fn spawn_udp_service(connection_id: String) {
    println!("started udp service for {}", connection_id);

    let _handle = std::thread::Builder::new()
        .name(connection_id.clone())
        .spawn(move || {
            // comment

            let mut udp_socket: Option<UdpSocket> = None;
            let mut channel_sender: Option<std::sync::mpsc::Sender<String>> = None;

            loop {
                let mut core_state = CORE_STATE.lock().unwrap();
                let udp_services = core_state.udp_services.clone();
                let udp_connections = core_state.main_state.udp_connections.clone();

                for service in udp_services {
                    let exists = udp_connections
                        .iter()
                        .any(|x| x.connection_id == service.connection_id);

                    if !exists {
                        for (index, _sv) in
                            core_state.udp_services.iter_mut().enumerate().filter(
                                |(_ind, sv_mut)| sv_mut.connection_id == service.connection_id,
                            )
                        {
                            println!("UDP KILLING {}", service.connection_id);

                            core_state.udp_services.remove(index);
                            return;
                        }
                    }
                }

                drop(core_state);

                let mut con_index = 0;

                for (index, con) in udp_connections.iter().enumerate() {
                    if con.connection_id == connection_id {
                        con_index = index;
                    }
                }

                let udp_con = &udp_connections[con_index];

                // println!(
                //     "UDP POLL CON {} -- OUT: {}",
                //     udp_con.connection_id,
                //     udp_con.out_queue.len()
                // );

                let connecting = udp_con.connecting;
                let disconnecting = udp_con.disconnecting;
                let connected = udp_con.connected;

                if disconnecting {
                    println!("UDP {} DISCONNECTING", connection_id);

                    channel_sender
                        .as_mut()
                        .unwrap()
                        .send("kill".to_string())
                        .unwrap();

                    let disconnected_msg = UdpDisconnectedMsg {
                        msg_type: MsgType::UDP_DISCONNECTED,
                        connection_id: udp_con.connection_id.clone(),
                    };

                    let txt = serde_json::to_string(&disconnected_msg).unwrap();
                    let msg = tungstenite::Message::Text(txt);

                    let mut core_state = CORE_STATE.lock().unwrap();
                    core_state
                        .session_websocket
                        .as_mut()
                        .unwrap()
                        .write_message(msg)
                        .unwrap();

                    udp_socket = None;
                } else if connecting && !connected {
                    println!("UDP {} CONNECTING", connection_id);

                    let (connected_succeded, mut new_socket) =
                        open_udp_connection(&udp_con.host_address, udp_con.connection_id.clone());

                    if connected_succeded {
                        new_socket
                            .as_mut()
                            .unwrap()
                            .set_nonblocking(true)
                            .expect("Failed to set non-blocking");
                    }

                    if !connected_succeded {
                        let mut core_state = CORE_STATE.lock().unwrap();

                        for (_index, udp_con) in core_state
                            .main_state
                            .udp_connections
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, sv)| sv.connection_id == connection_id)
                        {
                            udp_con.connecting = false;
                        }

                        continue;
                    }

                    let connected_msg = UdpConnectedMsg {
                        msg_type: MsgType::UDP_CONNECTED,
                        connection_id: udp_con.connection_id.clone(),
                    };

                    let txt = serde_json::to_string(&connected_msg).unwrap();
                    let msg = tungstenite::Message::Text(txt);

                    let mut core_state = CORE_STATE.lock().unwrap();
                    core_state
                        .session_websocket
                        .as_mut()
                        .unwrap()
                        .write_message(msg)
                        .unwrap();

                    udp_socket = new_socket;

                    let (sender, receiver) = std::sync::mpsc::channel();

                    channel_sender = Some(sender);

                    spawn_read_service(
                        udp_socket.as_mut().unwrap().try_clone().unwrap(),
                        receiver,
                        connection_id.clone(),
                    );

                    for (_index, udp_con) in core_state
                        .main_state
                        .udp_connections
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, sv)| sv.connection_id == connection_id)
                    {
                        udp_con.connecting = false;
                    }
                } else if connected {
                    for out_msg in udp_con.out_queue.clone() {
                        println!("UDP OUT MSG: {:?}", out_msg.data);

                        // let txt = serde_json::to_string(&out_msg.txt).unwrap();
                        // let msg = tungstenite::Message::Text(txt);

                        udp_socket
                            .as_mut()
                            .unwrap()
                            .send_to(&out_msg.data, out_msg.peer_address.clone())
                            .unwrap();

                        let mut new_msg = UdpMessage::new();
                        new_msg.timestamp = utils::get_timestamp();
                        new_msg.msg_type = UdpMsgType::OUT;
                        new_msg.data = out_msg.data;
                        new_msg.msg_id = out_msg.msg_id;

                        let msg_sent = UdpSentMsg {
                            msg_type: MsgType::UDP_MSG_SENT,
                            connection_id: connection_id.clone(),
                            msg: new_msg,
                        };

                        let sent_txt = serde_json::to_string(&msg_sent).unwrap();
                        let sent_msg = tungstenite::Message::Text(sent_txt);

                        let mut core_state = CORE_STATE.lock().unwrap();
                        core_state
                            .session_websocket
                            .as_mut()
                            .unwrap()
                            .write_message(sent_msg)
                            .unwrap();
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(UDP_SERVICE_REFRESH_RATE));
            }
        })
        .unwrap();
}

pub fn spawn_read_service(
    current_udp: UdpSocket,
    channel_receiver: std::sync::mpsc::Receiver<String>,
    connection_id: String,
) {
    let con_id = connection_id.clone();

    let _handle = std::thread::Builder::new()
        .name(con_id.clone())
        .spawn(move || {
            // Buffer to store received data
            let mut buf: [u8; 1024] = [0; 1024];

            let mut kill_read_service = false;

            loop {
                // std::thread::sleep(std::time::Duration::from_millis(UDP_SERVICE_REFRESH_RATE));

                let channel_message =
                    match channel_receiver.recv_timeout(std::time::Duration::from_millis(400)) {
                        Ok(message) => message,
                        Err(_) => "timeout".to_string(),
                    };

                if channel_message == "kill" {
                    kill_read_service = true;
                }

                if kill_read_service {
                    break;
                }

                match current_udp.recv_from(&mut buf) {
                    Ok((_received_bytes, peer_addr)) => {
                        // println!("UDP RECEIVED");

                        let mut core_state = CORE_STATE.lock().unwrap();

                        let mut new_msg = UdpMessage::new();
                        new_msg.msg_type = UdpMsgType::IN;
                        new_msg.data = buf.to_vec();
                        new_msg.peer_address = peer_addr.to_string();
                        new_msg.timestamp = utils::get_timestamp();

                        let out = UdpReceivedMsg {
                            msg_type: MsgType::UDP_RECEIVED_MSG,
                            connection_id: con_id.clone(),
                            msg: new_msg,
                        };

                        let out_txt = serde_json::to_string(&out).unwrap();
                        let out_msg = tungstenite::Message::Text(out_txt);

                        core_state
                            .session_websocket
                            .as_mut()
                            .unwrap()
                            .write_message(out_msg)
                            .unwrap();
                    }

                    Err(err) => {
                        // println!("UDP FAILED TO READ -> {err}");

                        // let mut core_state = CORE_STATE.lock().unwrap();

                        // let disconnected_msg = UdpDisconnectedMsg {
                        //     msg_type: MsgType::UDP_DISCONNECTED,
                        //     connection_id: connection_id.clone(),
                        // };

                        // let txt = serde_json::to_string(&disconnected_msg).unwrap();
                        // let msg = tungstenite::Message::Text(txt);

                        // core_state
                        //     .session_websocket
                        //     .as_mut()
                        //     .unwrap()
                        //     .write_message(msg)
                        //     .unwrap();

                        // break;
                    }
                }
            }
        });
}

pub fn open_udp_connection(
    host_address: &String,
    connection_id: String,
) -> (bool, Option<UdpSocket>) {
    match UdpSocket::bind(host_address) {
        Ok(socket) => return (true, Some(socket)),

        Err(err) => {
            let mut core_state = CORE_STATE.lock().unwrap();

            let failed_msg = UdpConnectionFailedMsg {
                msg_type: MsgType::UDP_CONNECTION_FAILED,
                connection_id: connection_id.clone(),
                reason: err.to_string(),
            };

            let txt = serde_json::to_string(&failed_msg).unwrap();
            let msg = tungstenite::Message::Text(txt);

            core_state
                .session_websocket
                .as_mut()
                .unwrap()
                .write_message(msg)
                .unwrap();

            return (false, None);
        }
    };

    // println!("Connected to the server");
}
