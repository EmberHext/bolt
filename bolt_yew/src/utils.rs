use crate::BoltContext;
use crate::Msg;
use crate::GLOBAL_STATE;

use futures::SinkExt;
use gloo_net::websocket::Message;
use wasm_bindgen::JsCast;

use syntect::highlighting::ThemeSet;
use syntect::highlighting::{Color, Theme};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use bolt_common::prelude::*;

pub fn _get_current_request(bctx: &mut BoltContext) -> &mut HttpRequest {
    let current = bctx.main_state.http_current;
    return &mut bctx.main_state.http_requests[current];
}

pub fn _bolt_log(log: &str) {
    let log = log.to_string();

    let msg = LogMsg {
        msg_type: MsgType::LOG,
        log,
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);
}

pub fn bolt_panic(log: &str) {
    let log = log.to_string();

    let msg = PanicMsg {
        msg_type: MsgType::PANIC,
        log: log.clone(),
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);

    panic!("{}", log);
}

pub fn open_link(link: String) {
    let msg = OpenLinkMsg {
        msg_type: MsgType::OPEN_LINK,
        link,
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);
}

pub fn send_ping() {
    let msg = PingMsg {
        msg_type: MsgType::PING,
        body: "piiiinggg".to_string(),
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);
}

pub fn ws_write(msg: String) {
    wasm_bindgen_futures::spawn_local(async move {
        let mut state = GLOBAL_STATE.lock().unwrap();
        let write = state.bctx.ws_tx.as_mut().unwrap();

        write.send(Message::Text(msg)).await.unwrap();
    });
}

pub fn save_state(bctx: &mut BoltContext) {
    let save = serde_json::to_string(&bctx.main_state).unwrap();

    let msg = SaveStateMsg {
        msg_type: MsgType::SAVE_STATE,
        save,
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);
}

pub fn set_save_state(state: String) {
    let new_state: MainState = serde_json::from_str(&state).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    global_state.bctx.main_state = new_state;

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

pub fn restore_state() {
    let msg = RestoreStateMsg {
        msg_type: MsgType::RESTORE_STATE,
        save: "".to_string(),
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);
}

pub fn _set_html(id: &str, content: String) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, id).unwrap();

    div.set_inner_html(&content);
}

pub fn _set_focus(id: &str) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, id).unwrap();

    let div = div.dyn_into::<web_sys::HtmlElement>().unwrap();

    div.focus().unwrap();
}

// pub fn stop_event_propagation() {
//     let document = web_sys::window().and_then(|w| w.document()).unwrap();

//     let event = document.create_event("event").unwrap();

//     let event_target = event.target().unwrap();

//     let stop_propagation = event_target.dyn_into::<js_sys::Function>().unwrap();

//     stop_propagation.call1(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::NULL);
// }

pub fn get_method() -> HttpMethod {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "methodselect").unwrap();

    let select = div.dyn_into::<web_sys::HtmlSelectElement>().unwrap();

    let value = select.value();

    match value.as_str() {
        "get" => HttpMethod::GET,
        "post" => HttpMethod::POST,
        "put" => HttpMethod::PUT,
        "delete" => HttpMethod::DELETE,
        "head" => HttpMethod::HEAD,
        "patch" => HttpMethod::PATCH,
        "options" => HttpMethod::OPTIONS,
        "connect" => HttpMethod::CONNECT,

        _ => {
            bolt_panic("invalid method");

            HttpMethod::GET
        }
    }
}

pub fn get_url() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "urlinput").unwrap();

    div.dyn_into::<web_sys::HtmlInputElement>().unwrap().value()
}

pub fn get_tcp_peer_url() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "urlinput").unwrap();

    div.dyn_into::<web_sys::HtmlInputElement>().unwrap().value()
}

pub fn get_udp_peer_url() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "udp-peer-urlinput").unwrap();

    div.dyn_into::<web_sys::HtmlInputElement>().unwrap().value()
}

pub fn get_body() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "reqbody").unwrap();

    div.dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .value()
}

pub fn get_tcp_out_txt() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "reqbody").unwrap();

    let data_txt = div
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .value();

    data_txt
}
pub fn get_tcp_out_data() -> Result<Vec<u8>, serde_json::Error> {
    let data_txt = get_tcp_out_txt();

    let data: Result<Vec<u8>, serde_json::Error> = serde_json::from_str(&data_txt);

    data
}

pub fn get_udp_out_txt() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "reqbody").unwrap();

    let data_txt = div
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .value();

    data_txt
}
pub fn get_udp_out_data() -> Result<Vec<u8>, serde_json::Error> {
    let data_txt = get_udp_out_txt();

    let data: Result<Vec<u8>, serde_json::Error> = serde_json::from_str(&data_txt);

    data
}

pub fn get_header(index: usize) -> Vec<String> {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let key =
        web_sys::Document::get_element_by_id(&doc, &("headerkey".to_string() + &index.to_string()))
            .unwrap();
    let value = web_sys::Document::get_element_by_id(
        &doc,
        &("headervalue".to_string() + &index.to_string()),
    )
    .unwrap();

    let key = key.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let value = value.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    vec![key.value(), value.value()]
}

pub fn get_param(index: usize) -> Vec<String> {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let key =
        web_sys::Document::get_element_by_id(&doc, &("paramkey".to_string() + &index.to_string()))
            .unwrap();
    let value = web_sys::Document::get_element_by_id(
        &doc,
        &("paramvalue".to_string() + &index.to_string()),
    )
    .unwrap();

    let key = key.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let value = value.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    vec![key.value(), value.value()]
}

pub fn format_json(data: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(data).unwrap();

    serde_json::to_string_pretty(&value).unwrap()
}

fn create_custom_theme() -> Theme {
    let mut theme = ThemeSet::load_defaults().themes["base16-eighties.dark"].clone();

    theme.settings.background = Some(Color {
        r: 3,
        g: 7,
        b: 13,
        a: 1,
    });

    theme
}

pub fn highlight_body(body: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme = create_custom_theme();
    let syntax = syntax_set.find_syntax_by_extension("json").unwrap();

    highlighted_html_for_string(body, &syntax_set, syntax, &theme).unwrap()
}

pub fn parse_url(url: String, params: Vec<Vec<String>>) -> String {
    let mut new_url = url;

    if !params.is_empty() && !params[0][0].is_empty() {
        new_url.push('?');
    }

    for (i, param) in params.iter().enumerate() {
        if param[0].is_empty() || param[1].is_empty() {
            continue;
        }

        new_url.push_str(&param[0]);
        new_url.push('=');
        new_url.push_str(&param[1]);

        if i != params.len() - 1 {
            new_url.push('&');
        }
    }

    new_url
}

pub fn copy_string_to_clipboard(value: String) {
    let msg = CopyClipboardMsg {
        msg_type: MsgType::COPY_CLIPBOARD,
        value,
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);
}
