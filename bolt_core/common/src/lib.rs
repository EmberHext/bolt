pub mod http;
pub mod udp;
pub mod ws;
pub mod collection;

pub mod prelude {
    pub use crate::http::*;
    pub use crate::udp::*;
    pub use crate::ws::*;
    pub use crate::collection::*;

    use serde::{Deserialize, Serialize};

    pub static VERSION: &str = "0.12.5";

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
    pub struct MainState {
        pub page: Page,

        pub http_current: usize,
        pub ws_current: usize,
        pub udp_current: usize,
        pub col_current: Vec<usize>,

        pub http_requests: Vec<HttpRequest>,
        pub ws_connections: Vec<WsConnection>,
        pub udp_connections: Vec<UdpConnection>,
        pub collections: Vec<Collection>,
    }

    impl MainState {
        pub fn new() -> Self {
            Self {
                page: Page::HttpPage,

                http_current: 0,
                ws_current: 0,
                udp_current: 0,
                col_current: vec![0, 0],

                http_requests: vec![HttpRequest::new()],
                ws_connections: vec![WsConnection::new()],
                udp_connections: vec![UdpConnection::new()],
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
        WS_CONNECTION_FAILED,

        ADD_UDP_CONNECTION,
        UDP_CONNECTED,
        UDP_DISCONNECTED,
        UDP_MSG_SENT,
        UDP_RECEIVED_MSG,
        UDP_CONNECTION_FAILED,
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

}
