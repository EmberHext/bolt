use crate::BoltContext;
use crate::Msg;
// use crate::SaveState;
use crate::GLOBAL_STATE;

use crate::receive_response;
use futures::SinkExt;
use gloo_net::websocket::Message;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, MouseEvent};

use syntect::highlighting::ThemeSet;
use syntect::highlighting::{Color, Theme};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use bolt_common::prelude::*;

pub fn _get_current_request(bctx: &mut BoltContext) -> &mut HttpRequest {
    let current = bctx.http_current;
    return &mut bctx.http_requests[current];
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

pub fn handle_ws_message(txt: String) {
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
            | MsgType::OPEN_LINK => {
                return;
            }

            MsgType::HTTP_RESPONSE => {
                handle_http_response_msg(txt);
            }

            MsgType::RESTORE_STATE => {
                handle_restore_response_msg(txt);
            }
        },

        Err(_err) => {
            handle_invalid_msg(txt);
        }
    }
}

fn handle_http_response_msg(txt: String) {
    // _bolt_log(&format!("received response"));

    receive_response(txt);
}

fn handle_ping_msg(_txt: String) {
    _bolt_log(&format!("received pong"));
}

fn handle_invalid_msg(txt: String) {
    _bolt_log(&format!("received invalid msg: {txt}"));
}

fn handle_restore_response_msg(txt: String) {
    // _bolt_log(&format!("received restore resp"));

    let msg: RestoreStateMsg = serde_json::from_str(&txt).unwrap();

    set_save_state(msg.save);
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

pub fn save_state(bctx: &mut BoltContext) {
    let save = serde_json::to_string(&bctx.main_state).unwrap();

    let msg = SaveStateMsg {
        msg_type: MsgType::SAVE_STATE,
        save,
    };

    let msg = serde_json::to_string(&msg).unwrap();

    ws_write(msg);
}

fn set_save_state(state: String) {
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

pub fn get_body() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "reqbody").unwrap();

    div.dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .value()
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

// HACK: disables selecting text
pub fn disable_text_selection() {
    if let Some(document) = web_sys::window().and_then(|win| win.document()) {
        if let Some(body) = document.body() {
            let listener = Closure::wrap(Box::new(move |event: MouseEvent| {
                event.prevent_default();
            }) as Box<dyn FnMut(_)>);
            let _ = EventTarget::from(body)
                .add_event_listener_with_callback("selectstart", listener.as_ref().unchecked_ref());
            listener.forget();
        }
    }
}

pub fn format_json(data: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(data).unwrap();

    serde_json::to_string_pretty(&value).unwrap()
}

fn create_custom_theme() -> Theme {
    let mut theme = ThemeSet::load_defaults().themes["base16-eighties.dark"].clone();

    // Change the background color
    theme.settings.background = Some(Color {
        r: 3,
        g: 7,
        b: 13,
        a: 1,
    });

    theme
}

pub fn highlight_body(body: &str) -> String {
    // Add syntax highlighting
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

    // bolt_log(&format!("url is: {new_url}"));
    new_url
}
