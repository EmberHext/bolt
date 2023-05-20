mod utils;

use bolt_common::prelude::*;
use std::sync::{Arc, Mutex};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, WebSocket};
use url::Url;

const WS_SERVICE_REFRESH_RATE: u64 = 500;
const SERVICE_SYNC_REFRESH_RATE: u64 = 1000;

// Create a shared global state variable
lazy_static::lazy_static! {
 static ref CORE_STATE: Arc<Mutex<CoreState>> = Arc::new(Mutex::new(CoreState::new()));
}

#[derive(Clone)]
struct WsService {
    connection_id: String,
}

pub struct CoreState {
    main_state: MainState,
    session_websocket: Option<WebSocket<std::net::TcpStream>>,
    ws_services: Vec<WsService>,
}

impl CoreState {
    pub fn new() -> Self {
        Self {
            main_state: MainState::new(),
            session_websocket: None,
            ws_services: vec![],
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

pub fn start_core_ws_service(_session_id: String) {
    std::thread::spawn(|| {
        // comment

        loop {
            let mut core_state = CORE_STATE.lock().unwrap();

            let connections = core_state.main_state.ws_connections.clone();
            let ws_services = core_state.ws_services.clone();

            for ws_con in connections.clone() {
                let exists = ws_services
                    .iter()
                    .any(|x| x.connection_id == ws_con.connection_id);

                if !exists {
                    spawn_ws_service(ws_con.connection_id.clone());

                    core_state.ws_services.push(WsService {
                        connection_id: ws_con.connection_id,
                    });
                }
            }

            drop(core_state);
            std::thread::sleep(std::time::Duration::from_millis(SERVICE_SYNC_REFRESH_RATE));
        }
    });
}

pub fn spawn_ws_service(connection_id: String) {
    // println!("started service for {}", connection_id);

    let _handle = std::thread::Builder::new()
        .name(connection_id.clone())
        .spawn(move || {
            // comment

            let mut socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>> = None;

            loop {
                let mut core_state = CORE_STATE.lock().unwrap();
                let ws_services = core_state.ws_services.clone();
                let ws_connections = core_state.main_state.ws_connections.clone();

                for service in ws_services {
                    let exists = ws_connections
                        .iter()
                        .any(|x| x.connection_id == service.connection_id);

                    if !exists {
                        for (index, _sv) in
                            core_state.ws_services.iter_mut().enumerate().filter(
                                |(_ind, sv_mut)| sv_mut.connection_id == service.connection_id,
                            )
                        {
                            println!("WS KILLING {}", service.connection_id);

                            core_state.ws_services.remove(index);
                            return;
                        }
                    }
                }

                drop(core_state);

                let mut con_index = 0;

                for (index, con) in ws_connections.iter().enumerate() {
                    if con.connection_id == connection_id {
                        con_index = index;
                    }
                }

                let ws_con = &ws_connections[con_index];

                // println!(
                //     "WS POLL CON {} -- OUT: {}",
                //     ws_con.connection_id,
                //     ws_con.out_queue.len()
                // );

                let connecting = ws_con.connecting;
                let disconnecting = ws_con.disconnecting;
                let connected = ws_con.connected;

                if disconnecting {
                    // println!("WS {} DISCONNECTING", connection_id);

                    socket.as_mut().unwrap().close(None).unwrap();

                    let disconnected_msg = WsDisconnectedMsg {
                        msg_type: MsgType::WS_DISCONNECTED,
                        connection_id: ws_con.connection_id.clone(),
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
                } else if connecting && !connected {
                    // println!("WS {} CONNECTING", connection_id);

                    let (connected_succeded, w_socket, _response) =
                        open_ws_connection(&ws_con.url, ws_con.connection_id.clone());

                    if !connected_succeded {
                        let mut core_state = CORE_STATE.lock().unwrap();

                        for (_index, ws_con) in core_state
                            .main_state
                            .ws_connections
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, sv)| sv.connection_id == connection_id)
                        {
                            ws_con.connecting = false;
                        }

                        continue;
                    }

                    let w_socket = w_socket.unwrap();
                    let _response = _response.unwrap();

                    let connected_msg = WsConnectedMsg {
                        msg_type: MsgType::WS_CONNECTED,
                        connection_id: ws_con.connection_id.clone(),
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

                    socket = Some(w_socket);

                    spawn_read_service(socket.as_mut().unwrap(), connection_id.clone());

                    for (_index, ws_con) in core_state
                        .main_state
                        .ws_connections
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, sv)| sv.connection_id == connection_id)
                    {
                        ws_con.connecting = false;
                    }
                } else if connected {
                    for out_msg in ws_con.out_queue.clone() {
                        // println!("OUT MSG: {}", out_msg.txt);

                        let txt = serde_json::to_string(&out_msg.txt).unwrap();
                        let msg = tungstenite::Message::Text(txt);

                        socket.as_mut().unwrap().write_message(msg).unwrap();

                        let mut new_msg = WsMessage::new();
                        new_msg.timestamp = utils::get_timestamp();
                        new_msg.msg_type = WsMsgType::OUT;
                        new_msg.txt = out_msg.txt;
                        new_msg.msg_id = out_msg.msg_id;

                        let msg_sent = WsSentMsg {
                            msg_type: MsgType::WS_MSG_SENT,
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

                std::thread::sleep(std::time::Duration::from_millis(WS_SERVICE_REFRESH_RATE));
            }
        })
        .unwrap();
}

pub fn spawn_read_service(
    current_ws: *const WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    connection_id: String,
) {
    let new_ws = unsafe { &mut *current_ws.cast_mut() };
    let con_id = connection_id.clone();

    let _handle = std::thread::Builder::new()
        .name(con_id.clone())
        .spawn(move || loop {
            match new_ws.read_message() {
                Ok(txt) => {
                    // println!("RECEIVED: {}", txt);

                    let mut core_state = CORE_STATE.lock().unwrap();

                    let mut new_msg = WsMessage::new();
                    new_msg.msg_type = WsMsgType::IN;
                    new_msg.txt = txt.into_text().unwrap();
                    new_msg.timestamp = utils::get_timestamp();

                    let out = WsReceivedMsg {
                        msg_type: MsgType::WS_RECEIVED_MSG,
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
                    println!("WS ERRO -> {err}");

                    let mut core_state = CORE_STATE.lock().unwrap();

                    let disconnected_msg = WsDisconnectedMsg {
                        msg_type: MsgType::WS_DISCONNECTED,
                        connection_id: connection_id.clone(),
                    };

                    let txt = serde_json::to_string(&disconnected_msg).unwrap();
                    let msg = tungstenite::Message::Text(txt);

                    core_state
                        .session_websocket
                        .as_mut()
                        .unwrap()
                        .write_message(msg)
                        .unwrap();

                    break;
                }
            }
        });
}

pub fn open_ws_connection(
    url: &String,
    connection_id: String,
) -> (
    bool,
    Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>,
    Option<tungstenite::http::Response<Option<Vec<u8>>>>,
) {
    match connect(Url::parse(url).unwrap()) {
        Ok((socket, response)) => return (true, Some(socket), Some(response)),

        Err(err) => {
            let mut core_state = CORE_STATE.lock().unwrap();

            let disconnected_msg = WsConnectionFailedMsg {
                msg_type: MsgType::WS_CONNECTION_FAILED,
                connection_id: connection_id.clone(),
                reason: err.to_string(),
            };

            let txt = serde_json::to_string(&disconnected_msg).unwrap();
            let msg = tungstenite::Message::Text(txt);

            core_state
                .session_websocket
                .as_mut()
                .unwrap()
                .write_message(msg)
                .unwrap();

            return (false, None, None);
        }
    };

    // println!("Connected to the server");
}

fn _close_ws_connection(socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>) {
    socket.close(None).unwrap();
}
