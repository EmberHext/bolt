use crate::utils::*;
use futures::stream::SplitSink;
use std::sync::{Arc, Mutex};
use yew::{html::Scope, Component, Context, Html};

use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message as WSMessage};

mod helpers;
mod process;
mod utils;
mod view;

use bolt_common::prelude::*;

// FIXME: request headers and params do not scroll

// Define the possible messages which can be sent to the component
#[derive(Clone)]
pub enum Msg {
    // HTTP
    HttpReqSelectedMethod(HttpMethod),
    SendHttpPressed,
    RemoveHttpRequest(usize),
    SelectHttpRequest(usize),
    AddHttpRequest,
    HttpReqParamChanged(usize),
    HttpReqBodyChanged,
    HttpReqHeaderChanged(usize),
    HttpReceivedResponse,
    HttpReqMethodChanged,
    HttpReqBodyPressed,
    HttpReqHeadersPressed,
    HttpReqParamsPressed,
    HttpRespBodyPressed,
    HttpRespHeadersPressed,
    HttpReqAddHeader,
    HttpReqRemoveHeader(usize),
    HttpReqAddParam,
    HttpReqRemoveParam(usize),
    CopyHttpResponsePressed,

    // WEBSOCKETS
    SendWsPressed,
    ConnectWsPressed,
    WsOutMessageChanged,
    WsOutMessagePressed,
    DisconnectWsPressed,
    AddWsConnection,
    RemoveWsConnection(usize),
    SelectWsConnection(usize),
    CopyWsMsgClicked(usize),

    // TCP
    SendTcpPressed,
    ConnectTcpPressed,
    TcpOutMessageChanged,
    TcpOutMessagePressed,
    TcpPeerUrlChanged,
    DisconnectTcpPressed,
    AddTcpConnection,
    RemoveTcpConnection(usize),
    SelectTcpConnection(usize),
    CopyTcpMsgClicked(usize),

    // UDP
    SendUdpPressed,
    ConnectUdpPressed,
    UdpOutMessageChanged,
    UdpOutMessagePressed,
    UdpPeerUrlChanged,
    DisconnectUdpPressed,
    AddUdpConnection,
    RemoveUdpConnection(usize),
    SelectUdpConnection(usize),
    CopyUdpMsgClicked(usize),

    // COLLECTION
    AddCollection,
    RemoveCollection(usize),
    AddToCollection(usize),
    SelectFromCollection(usize, usize),
    RemoveFromCollection(usize, usize),

    // OTHER
    UrlChanged,
    ToggleCollapsed(usize),
    Update,
    HelpPressed,
    GithubPressed,
    SwitchPage(Page),
    Nothing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoltApp {}

pub struct BoltState {
    bctx: BoltContext,
}

pub struct BoltContext {
    main_state: MainState,

    link: Option<Scope<BoltApp>>,

    ws_tx: Option<SplitSink<gloo_net::websocket::futures::WebSocket, WSMessage>>,
}

impl BoltContext {
    fn new() -> Self {
        BoltContext {
            main_state: MainState::new(),

            link: None,

            ws_tx: None,
        }
    }
}

unsafe impl Sync for BoltState {}
unsafe impl Send for BoltState {}

impl BoltState {
    fn new() -> Self {
        Self {
            bctx: BoltContext::new(),
        }
    }
}

lazy_static::lazy_static! {
    static ref GLOBAL_STATE: Arc<Mutex<BoltState>> = Arc::new(Mutex::new(BoltState::new()));
}

static BACKEND_WS: &str = "ws://127.0.0.1";
static WS_PORT: u16 = 3344;

fn main() {
    yew::Renderer::<BoltApp>::new().render();
}

impl Component for BoltApp {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.bctx.link = Some(ctx.link().clone());

        init(&mut state.bctx);

        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut state = GLOBAL_STATE.lock().unwrap();

        let should_render = process::update::process(&mut state.bctx, msg);

        if should_render {
            save_state(&mut state.bctx);
        }

        should_render
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut state = GLOBAL_STATE.lock().unwrap();

        let page = state.bctx.main_state.page;

        if page == Page::HttpPage {
            view::http::http_view(&mut state.bctx)
        } else if page == Page::Collections {
            view::collections::collections_view(&mut state.bctx)
        } else if page == Page::Tcp {
            view::tcp::tcp_view(&mut state.bctx)
        } else if page == Page::Udp {
            view::udp::udp_view(&mut state.bctx)
        } else if page == Page::Websockets {
            view::websockets::websockets_view(&mut state.bctx)
        } else if page == Page::Servers {
            view::servers::servers_view(&mut state.bctx)
        } else {
            view::http::http_view(&mut state.bctx)
        }
    }
}

fn init(bctx: &mut BoltContext) {
    // disable_text_selection();

    bctx.main_state.http_requests.push(HttpRequest::new());

    let ws = WebSocket::open(&(BACKEND_WS.to_string() + ":" + &WS_PORT.to_string())).unwrap();
    let (write, mut read) = ws.split();

    bctx.ws_tx = Some(write);

    wasm_bindgen_futures::spawn_local(async move {
        while let Some(msg) = read.next().await {
            let txt = msg.unwrap();

            let txt = match txt {
                WSMessage::Text(txt) => txt,
                WSMessage::Bytes(_) => panic!("got bytes in txt"),
            };

            handle_core_message(txt);
        }
        _bolt_log("WS: WebSocket Closed");
    });

    restore_state();
}

fn send_http_request(request: &mut HttpRequest) {
    request.loading = true;
    invoke_send(request);
}

fn connect_ws(connection: &mut WsConnection) {
    connection.connecting = true;
}

fn disconnect_ws(connection: &mut WsConnection) {
    connection.disconnecting = true;
}

fn send_ws(connection: &mut WsConnection) {
    let mut msg = WsMessage::new();
    msg.txt = get_body();
    msg.msg_type = WsMsgType::OUT;

    connection.out_queue.push(msg);
}

fn connect_tcp(connection: &mut TcpConnection) {
    connection.connecting = true;
}

fn disconnect_tcp(connection: &mut TcpConnection) {
    connection.disconnecting = true;
}

fn send_tcp(bctx: &mut BoltContext) {
    let data = get_tcp_out_data();
    match data {
        Ok(data) => {
            let connection = &mut bctx.main_state.tcp_connections[bctx.main_state.tcp_current];
            connection.failed = false;

            let mut msg = TcpMessage::new();
            msg.peer_address = get_tcp_peer_url();
            msg.msg_type = TcpMsgType::OUT;
            msg.data = data;

            connection.out_queue.push(msg);
        }

        Err(err) => {
            bctx.main_state.tcp_connections[bctx.main_state.tcp_current].failed = true;
            bctx.main_state.tcp_connections[bctx.main_state.tcp_current].failed_reason =
                "Error while parsing OUT data: ".to_string() + &err.to_string();
        }
    }
}

fn connect_udp(connection: &mut UdpConnection) {
    connection.connecting = true;
}

fn disconnect_udp(connection: &mut UdpConnection) {
    connection.disconnecting = true;
}

fn send_udp(bctx: &mut BoltContext) {
    let data = get_udp_out_data();
    match data {
        Ok(data) => {
            let connection = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];
            connection.failed = false;

            let mut msg = UdpMessage::new();
            msg.peer_address = get_udp_peer_url();
            msg.msg_type = UdpMsgType::OUT;
            msg.data = data;

            connection.out_queue.push(msg);
        }

        Err(err) => {
            bctx.main_state.udp_connections[bctx.main_state.udp_current].failed = true;
            bctx.main_state.udp_connections[bctx.main_state.udp_current].failed_reason =
                "Error while parsing OUT data: ".to_string() + &err.to_string();
        }
    }
}

pub fn http_receive_response(data: String) {
    let mut state = GLOBAL_STATE.lock().unwrap();
    let bctx = &mut state.bctx;

    let mut response: HttpResponse = serde_json::from_str(&data).unwrap();

    if response.response_type == HttpResponseType::JSON {
        response.body = format_json(&response.body);
        response.body_highlight = highlight_body(&response.body);
    }

    let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
    current.response = response;
    current.loading = false;

    let link = state.bctx.link.as_ref().unwrap();

    link.send_message(Msg::Update);
}

pub fn handle_core_message(txt: String) {
    let rcv: Result<ReceivedMessage, serde_json::Error> = serde_json::from_str(&txt);

    match rcv {
        Ok(message) => match message.msg_type {
            MsgType::PING => {
                handle_ping_msg(txt);
            }

            MsgType::SEND_HTTP
            | MsgType::SAVE_STATE
            | MsgType::LOG
            | MsgType::PANIC
            | MsgType::OPEN_LINK
            | MsgType::ADD_UDP_CONNECTION
            | MsgType::ADD_TCP_CONNECTION
            | MsgType::ADD_WS_CONNECTION
            | MsgType::COPY_CLIPBOARD => {
                return;
            }

            MsgType::HTTP_RESPONSE => {
                handle_http_response_msg(txt);
            }

            MsgType::RESTORE_STATE => {
                handle_restore_response_msg(txt);
            }

            MsgType::WS_CONNECTED => {
                handle_ws_connected_msg(txt);
            }
            MsgType::WS_DISCONNECTED => {
                handle_ws_disconnected_msg(txt);
            }
            MsgType::WS_CONNECTION_FAILED => {
                handle_ws_connection_failed_msg(txt);
            }
            MsgType::WS_MSG_SENT => {
                handle_ws_sent_msg(txt);
            }
            MsgType::WS_RECEIVED_MSG => {
                handle_ws_received_msg(txt);
            }

            MsgType::TCP_CONNECTED => {
                handle_tcp_connected_msg(txt);
            }
            MsgType::TCP_DISCONNECTED => {
                handle_tcp_disconnected_msg(txt);
            }
            MsgType::TCP_CONNECTION_FAILED => {
                handle_tcp_connection_failed_msg(txt);
            }
            MsgType::TCP_MSG_SENT => {
                handle_tcp_sent_msg(txt);
            }
            MsgType::TCP_RECEIVED_MSG => {
                handle_tcp_received_msg(txt);
            }

            MsgType::UDP_CONNECTED => {
                handle_udp_connected_msg(txt);
            }
            MsgType::UDP_DISCONNECTED => {
                handle_udp_disconnected_msg(txt);
            }
            MsgType::UDP_CONNECTION_FAILED => {
                handle_udp_connection_failed_msg(txt);
            }
            MsgType::UDP_MSG_SENT => {
                handle_udp_sent_msg(txt);
            }
            MsgType::UDP_RECEIVED_MSG => {
                handle_udp_received_msg(txt);
            }
        },

        Err(_err) => {
            handle_invalid_msg(txt);
        }
    }
}

fn handle_tcp_connected_msg(txt: String) {
    let msg: TcpConnectedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.tcp_connections {
        if msg.connection_id == con.connection_id {
            con.failed = false;
            con.connecting = false;
            con.connected = true;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_tcp_disconnected_msg(txt: String) {
    let msg: TcpDisconnectedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.tcp_connections {
        if msg.connection_id == con.connection_id {
            con.disconnecting = false;
            con.connecting = false;
            con.connected = false;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_tcp_connection_failed_msg(txt: String) {
    let msg: TcpConnectionFailedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.tcp_connections {
        if msg.connection_id == con.connection_id {
            con.failed = true;
            con.failed_reason = msg.reason.clone();
            con.disconnecting = false;
            con.connecting = false;
            con.connected = false;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_tcp_sent_msg(txt: String) {
    let sent_msg: TcpSentMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.tcp_connections {
        if sent_msg.connection_id == con.connection_id {
            for (index, out_msg) in con.out_queue.clone().iter().enumerate() {
                if out_msg.msg_id == sent_msg.msg.msg_id {
                    con.msg_history.push(sent_msg.msg.clone());
                    con.out_queue.remove(index);
                }
            }
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_tcp_received_msg(txt: String) {
    let received_msg: TcpReceivedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.tcp_connections {
        if con.connection_id == received_msg.connection_id {
            con.msg_history.push(received_msg.msg.clone());
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_udp_connected_msg(txt: String) {
    let msg: UdpConnectedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.udp_connections {
        if msg.connection_id == con.connection_id {
            con.failed = false;
            con.connecting = false;
            con.connected = true;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_udp_disconnected_msg(txt: String) {
    let msg: UdpDisconnectedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.udp_connections {
        if msg.connection_id == con.connection_id {
            con.disconnecting = false;
            con.connecting = false;
            con.connected = false;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_udp_connection_failed_msg(txt: String) {
    let msg: UdpConnectionFailedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.udp_connections {
        if msg.connection_id == con.connection_id {
            con.failed = true;
            con.failed_reason = msg.reason.clone();
            con.disconnecting = false;
            con.connecting = false;
            con.connected = false;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_udp_sent_msg(txt: String) {
    let sent_msg: UdpSentMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.udp_connections {
        if sent_msg.connection_id == con.connection_id {
            for (index, out_msg) in con.out_queue.clone().iter().enumerate() {
                if out_msg.msg_id == sent_msg.msg.msg_id {
                    con.msg_history.push(sent_msg.msg.clone());
                    con.out_queue.remove(index);
                }
            }
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_udp_received_msg(txt: String) {
    let received_msg: UdpReceivedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.udp_connections {
        if con.connection_id == received_msg.connection_id {
            con.msg_history.push(received_msg.msg.clone());
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_ws_connected_msg(txt: String) {
    let msg: WsConnectedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.ws_connections {
        if msg.connection_id == con.connection_id {
            con.failed = false;
            con.connecting = false;
            con.connected = true;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_ws_disconnected_msg(txt: String) {
    let msg: WsDisconnectedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.ws_connections {
        if msg.connection_id == con.connection_id {
            con.disconnecting = false;
            con.connecting = false;
            con.connected = false;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_ws_connection_failed_msg(txt: String) {
    let msg: WsConnectionFailedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.ws_connections {
        if msg.connection_id == con.connection_id {
            con.failed = true;
            con.failed_reason = msg.reason.clone();
            con.disconnecting = false;
            con.connecting = false;
            con.connected = false;
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_ws_sent_msg(txt: String) {
    let sent_msg: WsSentMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.ws_connections {
        if sent_msg.connection_id == con.connection_id {
            for (index, out_msg) in con.out_queue.clone().iter().enumerate() {
                if out_msg.msg_id == sent_msg.msg.msg_id {
                    con.msg_history.push(sent_msg.msg.clone());
                    con.out_queue.remove(index);
                }
            }
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_ws_received_msg(txt: String) {
    let received_msg: WsReceivedMsg = serde_json::from_str(&txt).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    for con in &mut global_state.bctx.main_state.ws_connections {
        if con.connection_id == received_msg.connection_id {
            con.msg_history.push(received_msg.msg.clone());
        }
    }

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

fn handle_http_response_msg(txt: String) {
    http_receive_response(txt);
}

fn handle_ping_msg(_txt: String) {}

fn handle_invalid_msg(txt: String) {
    _bolt_log(&format!("received invalid msg: {txt}"));
}

fn handle_restore_response_msg(txt: String) {
    let msg: RestoreStateMsg = serde_json::from_str(&txt).unwrap();

    utils::set_save_state(msg.save);
}

pub fn invoke_send(request: &mut HttpRequest) {
    let msg = SendHttpMsg {
        msg_type: MsgType::SEND_HTTP,
        url: parse_url(request.url.clone(), request.params.clone()),
        method: request.method,
        body: request.body.clone(),
        headers: request.headers.clone(),
        index: request.response.request_index,
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);

    send_ping();
}
