use serde::{Deserialize, Serialize};
use crate::prelude::MsgType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TcpMsgType {
    IN,
    OUT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpMessage {
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub msg_id: String,
    pub msg_type: TcpMsgType,
    pub peer_address: String
}

impl TcpMessage {
    pub fn new() -> Self {
        let msg_id = uuid::Uuid::new_v4()
            .to_string()
            .splitn(2, '-')
            .next()
            .unwrap()
            .to_string();

        Self {
            data: vec![],
            timestamp: 0,
            msg_id,
            msg_type: TcpMsgType::OUT,
            peer_address: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConnection {
    pub connection_id: String,
    // pub host_address: String,
    pub peer_address: String,
    pub name: String,

    pub out_tab: u8,
    pub in_tab: u8,

    pub connecting: bool,
    pub disconnecting: bool,
    pub failed: bool,
    pub failed_reason: String,
    pub connected: bool,

    pub out_data_buffer: String,
    pub out_queue: Vec<TcpMessage>,
    pub out_headers: Vec<Vec<String>>,
    pub out_params: Vec<Vec<String>>,

    pub in_queue: Vec<TcpMessage>,

    pub msg_history: Vec<TcpMessage>,
}

impl TcpConnection {
    pub fn new() -> Self {
        let con_id = uuid::Uuid::new_v4()
            .to_string()
            .splitn(2, '-')
            .next()
            .unwrap()
            .to_string();

        Self {
            connection_id: con_id,
            // host_address: String::new(),
            peer_address: String::new(),
            name: "TCP connection ".to_string(),
            connecting: false,
            disconnecting: false,
            failed: false,
            failed_reason: String::new(),
            connected: false,

            out_tab: 1,
            in_tab: 1,

            out_data_buffer: String::new(),
            out_queue: vec![],
            out_headers: vec![vec![String::new(), String::new()]],
            out_params: vec![vec![String::new(), String::new()]],

            in_queue: vec![],

            msg_history: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddTcpConnectionMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TcpConnectedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TcpDisconnectedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TcpSentMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub msg: TcpMessage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TcpReceivedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub msg: TcpMessage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TcpConnectionFailedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub reason: String,
}
