use clipboard::ClipboardProvider;
use std::{
    net::{TcpListener, TcpStream},
    thread::spawn,
};
use tungstenite::Message;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    WebSocket,
};

use crate::session::utils::*;
use bolt_common::prelude::*;

fn process_message(websocket: &mut WebSocket<TcpStream>, session_id: &String, msg: Message) {
    // println!("WS {}: new message", session_id);

    if msg.is_text() {
        let txt = msg.into_text().unwrap();

        let rcv: Result<ReceivedMessage, serde_json::Error> = serde_json::from_str(&txt);

        match rcv {
            Ok(message) => match message.msg_type {
                MsgType::PING => {
                    handle_ping(websocket, session_id, txt);
                }

                MsgType::LOG => {
                    handle_log(websocket, session_id, txt);
                }

                MsgType::PANIC => {
                    handle_panic(websocket, session_id, txt);
                }

                MsgType::OPEN_LINK => {
                    handle_open_link(websocket, session_id, txt);
                }

                MsgType::COPY_CLIPBOARD => {
                    handle_copy_clipboard(websocket, session_id, txt);
                }

                MsgType::SAVE_STATE => {
                    handle_save_state(websocket, session_id, txt);
                }

                MsgType::SEND_HTTP => {
                    handle_send_http(websocket, session_id, txt);
                }

                MsgType::RESTORE_STATE => {
                    handle_restore_state(websocket, session_id, txt);
                }

                MsgType::HTTP_RESPONSE
                | MsgType::WS_CONNECTED
                | MsgType::WS_DISCONNECTED
                | MsgType::WS_MSG_SENT
                | MsgType::WS_RECEIVED_MSG
                | MsgType::WS_CONNECTION_FAILED
                | MsgType::TCP_CONNECTED
                | MsgType::TCP_DISCONNECTED
                | MsgType::TCP_MSG_SENT
                | MsgType::TCP_RECEIVED_MSG
                | MsgType::TCP_CONNECTION_FAILED
                | MsgType::UDP_CONNECTED
                | MsgType::UDP_DISCONNECTED
                | MsgType::UDP_MSG_SENT
                | MsgType::UDP_RECEIVED_MSG
                | MsgType::UDP_CONNECTION_FAILED => {
                    return;
                }

                MsgType::ADD_WS_CONNECTION => {
                    handle_add_ws_connection(websocket, session_id, txt);
                }

                MsgType::ADD_UDP_CONNECTION => {
                    handle_add_udp_connection(websocket, session_id, txt);
                }
                MsgType::ADD_TCP_CONNECTION => {
                    handle_add_tcp_connection(websocket, session_id, txt);
                }
            },

            Err(_err) => {
                handle_invalid(websocket, session_id, txt);
            }
        }
    } else {
    }
}

fn handle_add_tcp_connection(
    _websocket: &mut WebSocket<TcpStream>,
    _session_id: &String,
    txt: String,
) {
    let _msg: AddTcpConnectionMsg = serde_json::from_str(&txt).unwrap();

    // println!("adding tcp connection with id: {}", &msg.connection_id);
}

fn handle_add_udp_connection(
    _websocket: &mut WebSocket<TcpStream>,
    _session_id: &String,
    txt: String,
) {
    let _msg: AddUdpConnectionMsg = serde_json::from_str(&txt).unwrap();

    // println!("adding udp connection with id: {}", &msg.connection_id);
}

fn handle_add_ws_connection(
    _websocket: &mut WebSocket<TcpStream>,
    _session_id: &String,
    txt: String,
) {
    let _msg: AddWsConnectionMsg = serde_json::from_str(&txt).unwrap();

    // println!("adding ws connection with id: {}", &msg.connection_id);
}

#[tokio::main]
async fn handle_send_http(websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    // println!("{txt}");

    let msg: SendHttpMsg = serde_json::from_str(&txt).unwrap();

    let request = SendHttpRequest {
        url: msg.url,
        method: msg.method,
        body: msg.body,
        headers: msg.headers,
        request_index: msg.index,
    };

    let resp = bolt_http::http_send(request).await;

    let response = serde_json::to_string(&resp).unwrap();

    // println!("{}", response);

    ws_write(websocket, response);
}

fn handle_save_state(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: SaveStateMsg = serde_json::from_str(&txt).unwrap();

    // println!("{}: saving state", _session_id);

    let client_state: MainState = serde_json::from_str(&msg.save).unwrap();

    let save_state = serde_json::to_string(&client_state).unwrap();
    std::fs::write(get_home() + "state.json", save_state).unwrap();

    bolt_ws::set_main_state(client_state.clone());
    bolt_udp::set_main_state(client_state.clone());
    bolt_tcp::set_main_state(client_state.clone());
}

fn handle_restore_state(websocket: &mut WebSocket<TcpStream>, _session_id: &String, _txt: String) {
    let save = std::fs::read_to_string(get_home() + "state.json").unwrap();

    let msg = RestoreStateMsg {
        msg_type: MsgType::RESTORE_STATE,
        save,
    };

    let response = serde_json::to_string(&msg).unwrap();

    ws_write(websocket, response);
}

fn handle_open_link(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: OpenLinkMsg = serde_json::from_str(&txt).unwrap();

    println!("opening {}", &msg.link);

    webbrowser::open(&msg.link).unwrap();
}

fn handle_copy_clipboard(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: CopyClipboardMsg = serde_json::from_str(&txt).unwrap();

    let mut clipboard_ctx: clipboard::ClipboardContext =
        clipboard::ClipboardProvider::new().unwrap();

    clipboard_ctx.set_contents(msg.value).unwrap();
}

fn handle_log(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: LogMsg = serde_json::from_str(&txt).unwrap();

    println!("LOG: {}", msg.log);
}

fn handle_panic(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: PanicMsg = serde_json::from_str(&txt).unwrap();

    println!("PANIC: {}", msg.log);
}

fn handle_ping(websocket: &mut WebSocket<TcpStream>, _session_id: &String, _txt: String) {
    // println!("{}: received ping", session_id);

    let msg = PingMsg {
        msg_type: MsgType::PING,
        body: "pong".to_string(),
    };

    let response = serde_json::to_string(&msg).unwrap();

    ws_write(websocket, response);
}

pub fn ws_write(websocket: &mut WebSocket<TcpStream>, txt: String) {
    let msg = Message::Text(txt);

    websocket.write_message(msg).unwrap();
}

fn handle_invalid(websocket: &mut WebSocket<TcpStream>, session_id: &String, _txt: String) {
    println!("{}: received invalid", session_id);

    let response = Message::Text("that was invalid".to_string());
    websocket.write_message(response).unwrap();
}

fn process_connection(_req: &Request, mut response: Response, _session_id: &String) -> Response {
    // println!(
    //     "WS: new session {} on path: {}",
    //     session_id,
    //     req.uri().path()
    // );

    // println!("The request's headers are:");
    // for (ref header, _value) in req.headers() {
    // println!("* {}", header);
    // }

    let headers = response.headers_mut();
    headers.append("CustomHeader", ":)".parse().unwrap());

    response
}

pub fn launch_core_server(port: u16, address: String) {
    // println!("Starting WS server on ws://{}:{}", address, port);

    let server = TcpListener::bind(address + ":" + &port.to_string()).unwrap();

    for mut stream in server.incoming() {
        spawn(move || {
            let session_id = uuid::Uuid::new_v4()
                .to_string()
                .splitn(2, '-')
                .next()
                .unwrap()
                .to_string();

            let callback = |req: &Request, response: Response| {
                let response = process_connection(req, response, &session_id);

                Ok(response)
            };

            let new_session_ws_for_ws = WebSocket::from_raw_socket(
                stream.as_mut().unwrap().try_clone().unwrap(),
                tungstenite::protocol::Role::Server,
                None,
            );

            let new_session_ws_for_udp = WebSocket::from_raw_socket(
                stream.as_mut().unwrap().try_clone().unwrap(),
                tungstenite::protocol::Role::Server,
                None,
            );

            let new_session_ws_for_tcp = WebSocket::from_raw_socket(
                stream.as_mut().unwrap().try_clone().unwrap(),
                tungstenite::protocol::Role::Server,
                None,
            );

            let mut session_websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            bolt_ws::set_session_websocket(new_session_ws_for_ws);
            bolt_udp::set_session_websocket(new_session_ws_for_udp);
            bolt_tcp::set_session_websocket(new_session_ws_for_tcp);

            crate::start_services(session_id.clone());

            loop {
                let msg = session_websocket.read_message();

                match msg {
                    Ok(msg) => {
                        process_message(&mut session_websocket, &session_id, msg);
                    }

                    Err(err) => {
                        println!("WS {}: {}", &session_id, err);

                        return;
                    }
                }
            }
        });
    }
}
