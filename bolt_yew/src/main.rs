use crate::utils::*;
use futures::stream::SplitSink;
// use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use yew::{html::Scope, Component, Context, Html};

use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message as WSMessage};

mod helpers;
mod process;
mod style;
mod utils;
mod view;

use bolt_common::prelude::*;

// TODO: Copy response body button
// FIXME: request headers and params do not scroll

// Define the possible messages which can be sent to the component
#[derive(Clone)]
pub enum Msg {
    HttpReqSelectedMethod(HttpMethod),

    SendHttpPressed,
    ConnectWsPressed,
    SendWsPressed,

    DisconnectWsPressed,

    HttpReqBodyPressed,
    HttpReqHeadersPressed,
    HttpReqParamsPressed,

    WsOutMessagePressed,
    WsOutHeadersPressed,
    WsOutParamsPressed,

    HttpRespBodyPressed,
    HttpRespHeadersPressed,

    HttpReqAddHeader,
    HttpReqRemoveHeader(usize),

    HttpReqAddParam,
    HttpReqRemoveParam(usize),

    WsOutAddHeader,
    WsOutRemoveHeader(usize),

    WsOutAddParam,
    WsOutRemoveParam(usize),

    HttpReceivedResponse,

    HttpReqMethodChanged,
    UrlChanged,

    HttpReqBodyChanged,
    WsOutMessageChanged,

    HttpReqHeaderChanged(usize),
    WsOutHeaderChanged(usize),

    HttpReqParamChanged(usize),
    WsOutParamChanged(usize),

    AddHttpRequest,
    AddWsConnection,

    RemoveHttpRequest(usize),
    SelectHttpRequest(usize),

    RemoveWsRequest(usize),
    SelectWsRequest(usize),

    AddCollection,
    RemoveCollection(usize),
    AddToCollection(usize),

    SelectFromCollection(usize, usize),
    RemoveFromCollection(usize, usize),

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

// #[derive(Clone)]
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

// unsafe impl Sync for BoltApp {}
// unsafe impl Send for BoltApp {}
unsafe impl Sync for BoltState {}
unsafe impl Send for BoltState {}

impl BoltState {
    fn new() -> Self {
        Self {
            bctx: BoltContext::new(),
        }
    }
}

// Create a shared global state variable
lazy_static::lazy_static! {
    static ref GLOBAL_STATE: Arc<Mutex<BoltState>> = Arc::new(Mutex::new(BoltState::new()));
}

// static BACKEND: &str = "http://127.0.0.1:3344/";
static BACKEND_WS: &str = "ws://127.0.0.1";
static WS_PORT: u16 = 3344;

impl Component for BoltApp {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        disable_text_selection();

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

            handle_ws_message(txt);
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

    // _bolt_log("connect ws was pressed");
}

fn disconnect_ws(connection: &mut WsConnection) {
    connection.disconnecting = true;

    _bolt_log("disconnect ws was pressed");
}

fn send_ws(_connection: &mut WsConnection) {
    _bolt_log("send ws was pressed");
}

pub fn receive_response(data: String) {
    let mut state = GLOBAL_STATE.lock().unwrap();
    let bctx = &mut state.bctx;

    // bolt_log("received a response");

    let mut response: HttpResponse = serde_json::from_str(&data).unwrap();

    // _bolt_log(&format!("{:?}", response));

    if response.response_type == HttpResponseType::JSON {
        response.body = format_json(&response.body);
        response.body = highlight_body(&response.body);
    }

    let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
    current.response = response;
    current.loading = false;

    let link = state.bctx.link.as_ref().unwrap();

    link.send_message(Msg::Update);
}

fn main() {
    yew::Renderer::<BoltApp>::new().render();
}
