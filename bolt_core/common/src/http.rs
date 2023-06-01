use crate::prelude::MsgType;
use serde::{Deserialize, Serialize};
use std::fmt;

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
pub enum HttpResponseType {
    TEXT,
    JSON,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub body_highlight: String,
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
            body_highlight: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: HttpResponseType::TEXT,
            request_index: 0,
            failed: false,
        }
    }
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
    pub body_highlight: String,
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
            body_highlight: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: SendHttpResponseType::TEXT,
            request_index: 0,
            failed: false,
        }
    }
}
