// pub mod prelude;
// mod session;

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

                MsgType::SAVE_STATE => {
                    handle_save_state(websocket, session_id, txt);
                }

                MsgType::SEND_HTTP => {
                    handle_send_http(websocket, session_id, txt);
                }

                MsgType::RESTORE_STATE => {
                    handle_restore_state(websocket, session_id, txt);
                }

                MsgType::HTTP_RESPONSE => {
                    return;
                }
            },

            Err(_err) => {
                handle_invalid(websocket, session_id, txt);
            }
        }
    } else {
    }
}

#[tokio::main(flavor = "current_thread")]
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

    let resp = http_send(request).await;

    let response = serde_json::to_string(&resp).unwrap();

    // println!("{}", response);

    ws_write(websocket, response);
}

async fn http_send(mut req: SendHttpRequest) -> SendHttpResponse {
    if !req.url.contains("http") {
        let new_url = "http://".to_string() + &req.url;

        req.url = new_url;
    }

    let mut request = prepare_request(req.clone());

    for h in req.headers {
        if h[0] != "" && h[1] != "" {
            request = request.header(h[0].clone(), h[1].clone());
        }
    }

    let start = get_timestamp();
    let response = request.send().await;
    let end = get_timestamp();

    let mut http_response = match response {
        Ok(resp) => {
            let mut new_response = SendHttpResponse::new();

            new_response.headers = extract_headers(resp.headers());
            new_response.status = resp.status().as_u16();
            new_response.time = (end - start) as u32;
            new_response.body = resp.text().await.unwrap();
            new_response.size = new_response.body.len() as u64;

            for header in &new_response.headers {
                if header[0] == "content-type" {
                    if header[1].contains("application/json") {
                        new_response.response_type = SendHttpResponseType::JSON;
                    }
                }
            }

            new_response
        }

        Err(err) => {
            let mut err_resp = SendHttpResponse::new();

            err_resp.failed = true;

            err_resp.body = err.to_string();

            err_resp
        }
    };

    http_response.request_index = req.request_index;

    return http_response;
}

fn handle_save_state(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: SaveStateMsg = serde_json::from_str(&txt).unwrap();

    // println!("{}: saving state", _session_id);

    std::fs::write(get_home() + "state.json", msg.save).unwrap();
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

fn handle_log(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: LogMsg = serde_json::from_str(&txt).unwrap();

    println!("LOG: {}", msg.log);
}

fn handle_panic(_websocket: &mut WebSocket<TcpStream>, _session_id: &String, txt: String) {
    let msg: PanicMsg = serde_json::from_str(&txt).unwrap();

    println!("PANIC: {}", msg.log);
}

fn handle_ping(websocket: &mut WebSocket<TcpStream>, session_id: &String, _txt: String) {
    println!("{}: received ping", session_id);

    let msg = PingMsg {
        msg_type: MsgType::PING,
        body: "pong".to_string(),
    };

    let response = serde_json::to_string(&msg).unwrap();

    ws_write(websocket, response);
}

fn ws_write(websocket: &mut WebSocket<TcpStream>, txt: String) {
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
    println!("Starting WS server on {} port {}", address, port);

    let server = TcpListener::bind(address + ":" + &port.to_string()).unwrap();

    for stream in server.incoming() {
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

            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message();

                match msg {
                    Ok(msg) => {
                        process_message(&mut websocket, &session_id, msg);
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
