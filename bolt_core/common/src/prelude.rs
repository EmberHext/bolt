use serde::{Deserialize, Serialize};
use std::fmt;

pub static VERSION: &str = "0.12.4";

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum HttpResponseType {
    TEXT,
    JSON,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub headers: Vec<Vec<String>>,
    pub time: u32,
    pub size: u64,
    pub response_type: HttpResponseType,
    pub request_index: usize,
    pub failed: bool,
}

impl HttpResponse {
    fn new() -> Self {
        HttpResponse {
            status: 0,
            body: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: HttpResponseType::TEXT,
            request_index: 0,
            failed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMsgType {
    IN,
    OUT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub txt: String,
    pub timestamp: u64,
    pub msg_id: String,
    pub msg_type: WsMsgType,
}

impl WsMessage {
    pub fn new() -> Self {
        let msg_id = uuid::Uuid::new_v4()
            .to_string()
            .splitn(2, '-')
            .next()
            .unwrap()
            .to_string();

        Self {
            txt: String::new(),
            timestamp: 0,
            msg_id,
            msg_type: WsMsgType::OUT,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsConnection {
    pub connection_id: String,
    pub url: String,
    pub name: String,

    pub out_tab: u8,
    pub in_tab: u8,

    pub connecting: bool,
    pub disconnecting: bool,
    pub failed: bool,
    pub connected: bool,

    pub out_buffer: String,
    pub out_queue: Vec<WsMessage>,
    pub out_headers: Vec<Vec<String>>,
    pub out_params: Vec<Vec<String>>,

    pub in_queue: Vec<WsMessage>,

    pub msg_history: Vec<WsMessage>,
}

impl WsConnection {
    pub fn new() -> Self {
        let con_id = uuid::Uuid::new_v4()
            .to_string()
            .splitn(2, '-')
            .next()
            .unwrap()
            .to_string();

        Self {
            connection_id: con_id,
            url: String::new(),
            name: "Ws connection ".to_string(),
            connecting: false,
            disconnecting: false,
            failed: false,
            connected: false,

            out_tab: 1,
            in_tab: 1,

            out_buffer: String::new(),
            out_queue: vec![],
            out_headers: vec![vec![String::new(), String::new()]],
            out_params: vec![vec![String::new(), String::new()]],

            in_queue: vec![],

            msg_history: vec![],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub body: String,
    pub headers: Vec<Vec<String>>,
    pub params: Vec<Vec<String>>,
    pub method: HttpMethod,

    pub response: HttpResponse,

    // META
    pub name: String,

    pub req_tab: u8,
    pub resp_tab: u8,

    pub loading: bool,
}

impl HttpRequest {
    pub fn new() -> HttpRequest {
        HttpRequest {
            url: String::new(),
            body: String::new(),
            headers: vec![vec![String::new(), String::new()]],
            params: vec![vec![String::new(), String::new()]],
            method: HttpMethod::GET,

            response: HttpResponse::new(),

            name: "New Request ".to_string(),

            req_tab: 1,
            resp_tab: 1,

            loading: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    HttpPage,
    Collections,
    Websockets,
    Tcp,
    Udp,
    Servers,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<HttpRequest>,
    pub collapsed: bool,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            name: "New Collection ".to_string(),
            requests: vec![],
            collapsed: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MainState {
    pub page: Page,

    pub http_current: usize,
    pub ws_current: usize,
    pub col_current: Vec<usize>,

    pub http_requests: Vec<HttpRequest>,
    pub ws_connections: Vec<WsConnection>,
    pub collections: Vec<Collection>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            page: Page::HttpPage,

            http_current: 0,
            ws_current: 0,
            col_current: vec![0, 0],

            http_requests: vec![HttpRequest::new()],
            ws_connections: vec![WsConnection::new()],
            collections: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReceivedMessage {
    pub msg_type: MsgType,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Serialize, Deserialize)]
pub enum MsgType {
    PING,
    LOG,
    PANIC,
    OPEN_LINK,
    SAVE_STATE,
    SEND_HTTP,
    HTTP_RESPONSE,
    RESTORE_STATE,
    ADD_WS_CONNECTION,
    WS_CONNECTED,
    WS_DISCONNECTED,
    WS_MSG_SENT,
    WS_RECEIVED_MSG,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
    OPTIONS,
    CONNECT,
}

impl HttpMethod {
    pub fn count() -> usize {
        8
    }
}

impl From<usize> for HttpMethod {
    fn from(index: usize) -> Self {
        match index {
            0 => HttpMethod::GET,
            1 => HttpMethod::POST,
            2 => HttpMethod::PUT,
            3 => HttpMethod::DELETE,
            4 => HttpMethod::HEAD,
            5 => HttpMethod::PATCH,
            6 => HttpMethod::OPTIONS,
            7 => HttpMethod::CONNECT,
            _ => panic!("Invalid index for HttpMethod"),
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
        }
    }
}

impl From<String> for HttpMethod {
    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "get" => HttpMethod::GET,
            "post" => HttpMethod::POST,
            "put" => HttpMethod::PUT,
            "delete" => HttpMethod::DELETE,
            "head" => HttpMethod::HEAD,
            "patch" => HttpMethod::PATCH,
            "options" => HttpMethod::OPTIONS,
            "connect" => HttpMethod::CONNECT,
            _ => panic!("Invalid value for HttpMethod"),
        }
    }
}

impl From<HttpMethod> for String {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::HEAD => "HEAD".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
            HttpMethod::OPTIONS => "OPTIONS".to_string(),
            HttpMethod::CONNECT => "CONNECT".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PingMsg {
    pub msg_type: MsgType,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogMsg {
    pub msg_type: MsgType,
    pub log: String,
}

#[derive(Serialize, Deserialize)]
pub struct PanicMsg {
    pub msg_type: MsgType,
    pub log: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenLinkMsg {
    pub msg_type: MsgType,
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct RestoreStateMsg {
    pub msg_type: MsgType,
    pub save: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaveStateMsg {
    pub msg_type: MsgType,
    pub save: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendHttpMsg {
    pub msg_type: MsgType,

    pub url: String,
    pub method: HttpMethod,
    pub body: String,
    pub headers: Vec<Vec<String>>,
    pub index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendHttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub body: String,
    pub headers: Vec<Vec<String>>,
    pub request_index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SendHttpResponseType {
    TEXT,
    JSON,
}

#[derive(Clone, Serialize)]
pub struct SendHttpResponse {
    pub msg_type: MsgType,
    pub status: u16,
    pub body: String,
    pub headers: Vec<Vec<String>>,
    pub time: u32,
    pub size: u64,
    pub response_type: SendHttpResponseType,
    pub request_index: usize,
    pub failed: bool,
}

impl SendHttpResponse {
    pub fn new() -> Self {
        SendHttpResponse {
            msg_type: MsgType::HTTP_RESPONSE,
            status: 0,
            body: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: SendHttpResponseType::TEXT,
            request_index: 0,
            failed: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddWsConnectionMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WsConnectedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WsDisconnectedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WsSentMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub msg: WsMessage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WsReceivedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub msg: WsMessage,
}
