use crate::utils::*;
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use yew::{html::Scope, Component, Context, Html};

use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message as WSMessage};

mod helpers;
mod process;
mod style;
mod utils;
mod view;

use bolt_common::prelude::HttpMethod;

// TODO: Copy response body button
// FIXME: request headers and params do not scroll

// Define the possible messages which can be sent to the component
#[derive(Clone)]
pub enum Msg {
    SelectedMethod(HttpMethod),
    SendPressed,
    ReqBodyPressed,
    ReqHeadersPressed,
    ReqParamsPressed,

    RespBodyPressed,
    RespHeadersPressed,

    AddHeader,
    RemoveHeader(usize),

    AddParam,
    RemoveParam(usize),

    ReceivedResponse,

    MethodChanged,
    UrlChanged,
    BodyChanged,
    HeaderChanged(usize),
    ParamChanged(usize),

    AddRequest,
    RemoveRequest(usize),
    SelectRequest(usize),

    AddCollection,
    RemoveCollection(usize),
    AddToCollection(usize),

    SelectFromCollection(usize, usize),
    RemoveFromCollection(usize, usize),

    ToggleCollapsed(usize),

    Update,
    HelpPressed,
    SwitchPage(Page),

    Nothing,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Collections,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    TEXT,
    JSON,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoltApp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    status: u16,
    body: String,
    headers: Vec<Vec<String>>,
    time: u32,
    size: u64,
    response_type: ResponseType,
    request_index: usize,
    failed: bool,
}

impl Response {
    fn new() -> Self {
        Response {
            status: 0,
            body: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: ResponseType::TEXT,
            request_index: 0,
            failed: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Request {
    url: String,
    body: String,
    headers: Vec<Vec<String>>,
    params: Vec<Vec<String>>,
    method: HttpMethod,

    response: Response,

    // META
    name: String,

    req_tab: u8,
    resp_tab: u8,

    loading: bool,
}

impl Request {
    fn new() -> Request {
        Request {
            url: String::new(),
            body: String::new(),
            headers: vec![vec![String::new(), String::new()]],
            params: vec![vec![String::new(), String::new()]],
            method: HttpMethod::GET,

            response: Response::new(),

            // META
            name: "New Request ".to_string(),

            req_tab: 1,
            resp_tab: 1,

            loading: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Collection {
    name: String,
    requests: Vec<Request>,
    collapsed: bool,
}

impl Collection {
    fn new() -> Collection {
        Collection {
            name: "New Collection ".to_string(),
            requests: vec![],
            collapsed: false,
        }
    }
}

pub struct BoltState {
    bctx: BoltContext,
}

// #[derive(Clone)]
pub struct BoltContext {
    link: Option<Scope<BoltApp>>,

    page: Page,
    main_current: usize,
    col_current: Vec<usize>,

    main_col: Collection,
    collections: Vec<Collection>,
    ws_tx: Option<SplitSink<gloo_net::websocket::futures::WebSocket, WSMessage>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SaveState {
    page: Page,

    main_current: usize,
    col_current: Vec<usize>,

    main_col: Collection,
    collections: Vec<Collection>,
}

impl BoltContext {
    fn new() -> Self {
        BoltContext {
            link: None,

            main_col: Collection::new(),
            collections: vec![],
            page: Page::Home,

            main_current: 0,
            col_current: vec![0, 0],
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

        state.bctx.main_col.requests.push(Request::new());

        let ws = WebSocket::open(&(BACKEND_WS.to_string() + ":" + &WS_PORT.to_string())).unwrap();
        let (write, mut read) = ws.split();

        state.bctx.ws_tx = Some(write);

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

        let page = state.bctx.page;

        if page == Page::Home {
            view::home::home_view(&mut state.bctx)
        } else if page == Page::Collections {
            view::collections::collections_view(&mut state.bctx)
        } else {
            view::home::home_view(&mut state.bctx)
        }
    }
}

fn send_request(request: &mut Request) {
    request.loading = true;
    invoke_send(request);
}

pub fn receive_response(data: String) {
    let mut state = GLOBAL_STATE.lock().unwrap();
    let bctx = &mut state.bctx;

    // bolt_log("received a response");

    let mut response: Response = serde_json::from_str(&data).unwrap();

    // _bolt_log(&format!("{:?}", response));

    if response.response_type == ResponseType::JSON {
        response.body = format_json(&response.body);
        response.body = highlight_body(&response.body);
    }

    if bctx.page == Page::Home {
        let current = response.request_index;
        state.bctx.main_col.requests[current].response = response;
        state.bctx.main_col.requests[current].loading = false;
    } else {
        let current = &bctx.col_current;
        bctx.collections[current[0]].requests[current[1]].response = response;
        bctx.collections[current[0]].requests[current[1]].loading = false;
    }

    let link = state.bctx.link.as_ref().unwrap();

    link.send_message(Msg::Update);
}

fn main() {
    yew::Renderer::<BoltApp>::new().render();
}
