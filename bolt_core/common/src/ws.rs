use serde::{Deserialize, Serialize};
use crate::prelude::MsgType;

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
    pub failed_reason: String,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct WsConnectionFailedMsg {
    pub msg_type: MsgType,
    pub connection_id: String,
    pub reason: String,
}
