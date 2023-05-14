mod session;
mod utils;

use std::sync::{Arc, Mutex};

use bolt_common::prelude::*;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, WebSocket};
use url::Url;

static VERSION: &str = "0.11.11";
static HELP: &str = r#"
Bolt CLI (Build and test APIs)

Usage:
  bolt [OPTIONS]...
  bolt -h | --help
  bolt -v | --version
Options:
  -h --help      Show this screen.
  -v --version   Show version.
  --reset        Reset static files
    "#;

static ADDRESS: &str = "127.0.0.1";

// Create a shared global state variable
lazy_static::lazy_static! {
    static ref CORE_STATE: Arc<Mutex<CoreState>> = Arc::new(Mutex::new(CoreState::new()));
}

#[derive(Clone)]
struct WsService {
    connection_id: String,
    kill: bool,
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

const SERVICE_SYNC_REFRESH_RATE: u64 = 1000;
const WS_SERVICE_REFRESH_RATE: u64 = 1000;

pub fn start(args: Vec<String>, port: u16) {
    let mut args = args;

    args.remove(0);

    let mut is_tauri = false;
    let mut is_headless = false;
    let mut launch = false;
    let mut reset = false;

    match std::env::var_os("BOLT_DEV") {
        Some(_) => {
            reset = true;
        }
        None => {}
    }

    if args.len() > 0 {
        let flag = args[0].as_str();

        match flag {
            "--reset" => reset = true,

            "-h" | "--help" => {
                println!("{}", HELP);
            }

            "-v" | "--version" => {
                println!("bolt {}", VERSION);
            }

            "--tauri" => {
                is_tauri = true;

                launch = true;
            }

            "--headless" => {
                is_headless = true;

                launch = true;
            }

            _ => {
                panic!("unknown flag");
            }
        }
    } else {
        launch = true;
    }

    if reset {
        utils::reset_home();
    }

    if launch {
        utils::verify_home();
        utils::verify_state();

        if !is_tauri {
            utils::verify_dist();
        }

        if !is_tauri && !is_headless {
            std::thread::spawn(move || {
                session::asset::launch_asset_server(port + 1, ADDRESS.to_string());

                std::process::exit(0);
            });
        }

        session::server::launch_core_server(port, ADDRESS.to_string());
    }
}

fn start_services(session_id: String) {
    println!("Starting core services");

    std::thread::spawn(move || {
        start_ws_service(session_id);
    });
}

fn start_ws_service(_session_id: String) {
    std::thread::spawn(|| {
        // comment

        loop {
            let mut core_state = CORE_STATE.lock().unwrap();

            let connections = core_state.main_state.ws_connections.clone();
            let ws_services = core_state.ws_services.clone();

            // println!("connections: {:?}", connections.len());
            // println!("services: {:?}", ws_services.len());

            for ws_con in connections.clone() {
                let exists = ws_services
                    .iter()
                    .any(|x| x.connection_id == ws_con.connection_id);

                if !exists {
                    spawn_ws_service(ws_con.connection_id.clone());

                    core_state.ws_services.push(WsService {
                        connection_id: ws_con.connection_id,
                        kill: false,
                    });
                }
            }

            for service in ws_services {
                let exists = connections
                    .iter()
                    .any(|x| x.connection_id == service.connection_id);

                if !exists {
                    for sv in core_state
                        .ws_services
                        .iter_mut()
                        .filter(|sv_mut| sv_mut.connection_id == service.connection_id)
                    {
                        sv.kill = true;
                    }
                }
            }

            drop(core_state);
            std::thread::sleep(std::time::Duration::from_millis(SERVICE_SYNC_REFRESH_RATE));
        }
    });
}

fn spawn_ws_service(connection_id: String) {
    // println!("started service for {}", connection_id);

    let _handle = std::thread::Builder::new()
        .name(connection_id.clone())
        .spawn(move || {
            // comment

            let mut socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>> = None;

            loop {
                let mut core_state = CORE_STATE.lock().unwrap();

                let mut kill_service = false;
                let mut service_index = 0;
                let mut con_index = 0;

                for (index, ws_service) in core_state
                    .ws_services
                    .iter()
                    .enumerate()
                    .filter(|(_, sv)| sv.connection_id == connection_id)
                {
                    if ws_service.kill {
                        // println!("KILLED {}", conn.connection_id);

                        service_index = index;
                        kill_service = true;
                    }
                }

                if kill_service {
                    core_state.ws_services.remove(service_index);
                    break;
                }

                for (index, con) in core_state.main_state.ws_connections.iter().enumerate() {
                    if con.connection_id == connection_id {
                        con_index = index;
                    }
                }

                let ws_con = &core_state.main_state.ws_connections[con_index];

                // println!(
                //     "POLL CON {} -- OUT: {}",
                //     ws_con.connection_id,
                //     ws_con.out_queue.len()
                // );

                let connecting = ws_con.connecting;
                let disconnecting = ws_con.disconnecting;
                let connected = ws_con.connected;

                if disconnecting {
                    println!("WS {} DISCONNECTING", connection_id);

                    socket.as_mut().unwrap().close(None).unwrap();

                    let disconnected_msg = WsDisconnectedMsg {
                        msg_type: MsgType::WS_DISCONNECTED,
                        connection_id: ws_con.connection_id.clone(),
                    };

                    let txt = serde_json::to_string(&disconnected_msg).unwrap();
                    let msg = tungstenite::Message::Text(txt);

                    core_state
                        .session_websocket
                        .as_mut()
                        .unwrap()
                        .write_message(msg)
                        .unwrap();
                } else if connecting {
                    println!("WS {} CONNECTING", connection_id);

                    let (w_socket, _response) = open_ws_connection(&ws_con.url);

                    // TODO: Notify client of successful connection

                    let connected_msg = WsConnectedMsg {
                        msg_type: MsgType::WS_CONNECTED,
                        connection_id: ws_con.connection_id.clone(),
                    };

                    let txt = serde_json::to_string(&connected_msg).unwrap();
                    let msg = tungstenite::Message::Text(txt);

                    core_state
                        .session_websocket
                        .as_mut()
                        .unwrap()
                        .write_message(msg)
                        .unwrap();

                    // crate::session::server::ws_write(core_state.session_websocket.as_mut().unwrap(), msg);

                    socket = Some(w_socket);

                    socket
                        .as_mut()
                        .unwrap()
                        .write_message(tungstenite::Message::Text("Hello WebSocket".into()))
                        .unwrap();
                } else if connected {
                    for out_msg in ws_con.out_queue.clone() {
                        // println!("OUT MSG: {}", out_msg.txt);

                        let txt = serde_json::to_string(&out_msg.txt).unwrap();
                        let msg = tungstenite::Message::Text(txt);

                        socket.as_mut().unwrap().write_message(msg).unwrap();

                        let msg_sent = WsSentMsg {
                            msg_type: MsgType::WS_MSG_SENT,
                            connection_id: connection_id.clone(),
                            msg_id: out_msg.msg_id.clone(),
                        };

                        let sent_txt = serde_json::to_string(&msg_sent).unwrap();
                        let sent_msg = tungstenite::Message::Text(sent_txt);

                        core_state
                            .session_websocket
                            .as_mut()
                            .unwrap()
                            .write_message(sent_msg)
                            .unwrap();
                    }
                }

                // TODO: dispatch in_queue
                // BUG: causes malfunction and thread panics
                // let msg = socket
                // .as_mut()
                // .unwrap()
                // .read_message()
                // .expect("Error reading message");
                // println!("Received: {}", msg);

                drop(core_state);
                std::thread::sleep(std::time::Duration::from_millis(WS_SERVICE_REFRESH_RATE));
            }

            // cleanup
            // close_ws_connection(socket.as_mut().unwrap());
        })
        .unwrap();
}

fn open_ws_connection(
    url: &String,
) -> (
    WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    tungstenite::http::Response<Option<Vec<u8>>>,
) {
    let (socket, response) = connect(Url::parse(url).unwrap()).expect("Can't connect");

    println!("Connected to the server");

    (socket, response)
}

fn close_ws_connection(socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>) {
    socket.close(None).unwrap();
}
