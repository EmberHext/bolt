use serde::{Deserialize, Serialize};
use std::fmt;

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
    RESTORE_STATE
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

