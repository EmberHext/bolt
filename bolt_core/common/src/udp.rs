use serde::{Deserialize, Serialize};
use crate::prelude::MsgType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UdpMsgType {
    IN,
    OUT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpMessage {
    pub txt: String,
    pub timestamp: u64,
    pub msg_id: String,
    pub msg_type: UdpMsgType,
}

impl UdpMessage {
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
            msg_type: UdpMsgType::OUT,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpConnection {
    pub connection_id: String,
    pub url: String,
    pub name: String,

    pub out_tab: u8,
    pub in_tab: u8,

    pub connecting: bool,
    pub disconnecting: bool,
    pub failed: bool,
    pub failed_reason: String,
    pub connected: bool,

    pub out_buffer: String,
    pub out_queue: Vec<UdpMessage>,
    pub out_headers: Vec<Vec<String>>,
    pub out_params: Vec<Vec<String>>,

    pub in_queue: Vec<UdpMessage>,

    pub msg_history: Vec<UdpMessage>,
}

impl UdpConnection {
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
            name: "UDP connection ".to_string(),
            connecting: false,
            disconnecting: false,
            failed: false,
            failed_reason: String::new(),
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

#[derive(Serialize, Deserialize, Clone)]
pub struct AddUdpConnectionMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UdpConnectedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UdpDisconnectedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UdpSentMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub msg: UdpMessage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UdpReceivedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub msg: UdpMessage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UdpConnectionFailedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub reason: String,
}
