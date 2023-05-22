use std::io::Read;
use std::io::Write;
mod utils;

use bolt_common::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tungstenite::WebSocket;

const TCP_SERVICE_REFRESH_RATE: u64 = 500;
const SERVICE_SYNC_REFRESH_RATE: u64 = 1000;

// Create a shared global state variable
lazy_static::lazy_static! {
 static ref CORE_STATE: Arc<Mutex<CoreState>> = Arc::new(Mutex::new(CoreState::new()));
}

#[derive(Clone)]
struct TcpService {
    connection_id: String,
}

pub struct CoreState {
    main_state: MainState,
    session_websocket: Option<WebSocket<std::net::TcpStream>>,
    tcp_services: Vec<TcpService>,
}

impl CoreState {
    pub fn new() -> Self {
        Self {
            main_state: MainState::new(),
            session_websocket: None,
            tcp_services: vec![],
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

pub fn start_core_tcp_service(_session_id: String) {
    std::thread::spawn(|| {
        // comment

        loop {
            let mut core_state = CORE_STATE.lock().unwrap();

            let connections = core_state.main_state.tcp_connections.clone();
            let tcp_services = core_state.tcp_services.clone();

            for tcp_con in connections.clone() {
                let exists = tcp_services
                    .iter()
                    .any(|x| x.connection_id == tcp_con.connection_id);

                if !exists {
                    spawn_tcp_service(tcp_con.connection_id.clone());

                    core_state.tcp_services.push(TcpService {
                        connection_id: tcp_con.connection_id,
                    });
                }
            }

            drop(core_state);
            std::thread::sleep(std::time::Duration::from_millis(SERVICE_SYNC_REFRESH_RATE));
        }
    });
}

pub fn spawn_tcp_service(connection_id: String) {
    // println!("started tcp service for {}", connection_id);

    let _handle = std::thread::Builder::new()
        .name(connection_id.clone())
        .spawn(move || {
            // comment

            let mut tcp_stream: Option<TcpStream> = None;
            let mut channel_sender: Option<std::sync::mpsc::Sender<String>> = None;

            loop {
                let mut core_state = CORE_STATE.lock().unwrap();
                let tcp_services = core_state.tcp_services.clone();
                let tcp_connections = core_state.main_state.tcp_connections.clone();

                for service in tcp_services {
                    let exists = tcp_connections
                        .iter()
                        .any(|x| x.connection_id == service.connection_id);

                    if !exists {
                        for (index, _sv) in
                            core_state.tcp_services.iter_mut().enumerate().filter(
                                |(_ind, sv_mut)| sv_mut.connection_id == service.connection_id,
                            )
                        {
                            // println!("TCP KILLING {}", service.connection_id);

                            core_state.tcp_services.remove(index);
                            return;
                        }
                    }
                }

                drop(core_state);

                let mut con_index = 0;

                for (index, con) in tcp_connections.iter().enumerate() {
                    if con.connection_id == connection_id {
                        con_index = index;
                    }
                }

                let tcp_con = &tcp_connections[con_index];

                // println!(
                //     "TCP POLL CON {} -- OUT: {}",
                //     tcp_con.connection_id,
                //     tcp_con.out_queue.len()
                // );

                let connecting = tcp_con.connecting;
                let disconnecting = tcp_con.disconnecting;
                let connected = tcp_con.connected;

                if disconnecting {
                    // println!("TCP {} DISCONNECTING", connection_id);

                    channel_sender
                        .as_mut()
                        .unwrap()
                        .send("kill".to_string())
                        .unwrap();

                    let disconnected_msg = TcpDisconnectedMsg {
                        msg_type: MsgType::TCP_DISCONNECTED,
                        connection_id: tcp_con.connection_id.clone(),
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

                    tcp_stream = None;
                } else if connecting && !connected {
                    // println!("TCP {} CONNECTING", connection_id);

                    let (connected_succeded, mut new_stream) =
                        open_tcp_connection(&tcp_con.peer_address, tcp_con.connection_id.clone());

                    if connected_succeded {
                        new_stream
                            .as_mut()
                            .unwrap()
                            .set_nonblocking(true)
                            .expect("Failed to set non-blocking");
                    }

                    if !connected_succeded {
                        let mut core_state = CORE_STATE.lock().unwrap();

                        for (_index, tcp_con) in core_state
                            .main_state
                            .tcp_connections
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, sv)| sv.connection_id == connection_id)
                        {
                            tcp_con.connecting = false;
                        }

                        continue;
                    }

                    let connected_msg = TcpConnectedMsg {
                        msg_type: MsgType::TCP_CONNECTED,
                        connection_id: tcp_con.connection_id.clone(),
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

                    tcp_stream = new_stream;

                    let (sender, receiver) = std::sync::mpsc::channel();

                    channel_sender = Some(sender);

                    spawn_read_service(
                        tcp_stream.as_mut().unwrap().try_clone().unwrap(),
                        receiver,
                        connection_id.clone(),
                    );

                    for (_index, tcp_con) in core_state
                        .main_state
                        .tcp_connections
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, sv)| sv.connection_id == connection_id)
                    {
                        tcp_con.connecting = false;
                    }
                } else if connected {
                    for out_msg in tcp_con.out_queue.clone() {
                        // println!("TCP OUT MSG: {:?}", out_msg.data);

                        tcp_stream
                            .as_mut()
                            .unwrap()
                            .write(&out_msg.data)
                            .unwrap();

                        let mut new_msg = TcpMessage::new();
                        new_msg.timestamp = utils::get_timestamp();
                        new_msg.msg_type = TcpMsgType::OUT;
                        new_msg.data = out_msg.data;
                        new_msg.msg_id = out_msg.msg_id;

                        let msg_sent = TcpSentMsg {
                            msg_type: MsgType::TCP_MSG_SENT,
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

                std::thread::sleep(std::time::Duration::from_millis(TCP_SERVICE_REFRESH_RATE));
            }
        })
        .unwrap();
}

pub fn spawn_read_service(
    mut current_tcp: TcpStream,
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

                match current_tcp.read(&mut buf) {
                    Ok(_received_bytes) => {
                        // println!("TCP RECEIVED");

                        let mut core_state = CORE_STATE.lock().unwrap();

                        let mut new_msg = TcpMessage::new();
                        new_msg.msg_type = TcpMsgType::IN;
                        new_msg.data = buf.to_vec();
                        // new_msg.peer_address = peer_addr.to_string();
                        new_msg.timestamp = utils::get_timestamp();

                        let out = TcpReceivedMsg {
                            msg_type: MsgType::TCP_RECEIVED_MSG,
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

                    Err(_err) => {}
                }
            }
        });
}

pub fn open_tcp_connection(
    peer_address: &String,
    connection_id: String,
) -> (bool, Option<TcpStream>) {
    match TcpStream::connect(peer_address) {
        Ok(stream) => return (true, Some(stream)),

        Err(err) => {
            let mut core_state = CORE_STATE.lock().unwrap();

            let failed_msg = TcpConnectionFailedMsg {
                msg_type: MsgType::TCP_CONNECTION_FAILED,
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
