use crate::BoltApp;
use crate::Msg;
use bolt_common::prelude::*;
use yew::html::Scope;
use yew::{html, Html};

pub fn render_ws_msg(msg: &WsMessage, link: &Scope<BoltApp>, index: usize) -> Html {
    let txt = msg.txt.clone();

    let txt = if txt.len() > 60 {
        format!("{}...", &txt[0..60])
    } else {
        txt
    };

    let time = format_time(msg.timestamp);

    let copy_icon = crate::view::icons::copy_icon(20, 20);

    match msg.msg_type {
        WsMsgType::IN => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-in-arrow">{"↓"}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
                        <div class="copy-msg-icon" title="copy message" onclick={link.callback(move |_| Msg::CopyWsMsgClicked(index))} >{copy_icon}</div>
                        {time}
                        <div class="ws-open-arrow">{"❯"}</div>
                    </div>
                </label>

                <div class="atab-content">
                  {msg.txt.clone()}
                </div>
              </div>
            }
        }

        WsMsgType::OUT => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-out-arrow">{"↑"}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
                        <div class="copy-msg-icon" title="copy message" onclick={link.callback(move |_| Msg::CopyWsMsgClicked(index))} >{copy_icon}</div>
                        {time}
                        <div class="ws-open-arrow">{"❯"}</div>
                    </div>
                </label>

                <div class="atab-content">
                  {msg.txt.clone()}
                </div>
              </div>
            }
        }
    }
}

pub fn render_tcp_msg(msg: &TcpMessage, link: &Scope<BoltApp>, index: usize) -> Html {
    let data = format!("{:?}", msg.data.clone());

    let txt = if data.len() > 60 {
        format!("{}...", &data[0..60])
    } else {
        data.clone()
    };

    let time = format_time(msg.timestamp);

    let copy_icon = crate::view::icons::copy_icon(20, 20);

    match msg.msg_type {
        TcpMsgType::IN => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-in-arrow">{"↓"}</div>
                        <div class="udp-msg-peer-address">{msg.peer_address.clone()}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
                        <div class="copy-msg-icon" title="copy message" onclick={link.callback(move |_| Msg::CopyTcpMsgClicked(index))} >{copy_icon}</div>
                        {time}
                        <div class="ws-open-arrow">{"❯"}</div>
                    </div>
                </label>

                <div class="atab-content">
                  {data.clone()}
                </div>
              </div>
            }
        }

        TcpMsgType::OUT => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-out-arrow">{"↑"}</div>
                        <div class="udp-msg-peer-address">{msg.peer_address.clone()}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
                        <div class="copy-msg-icon" title="copy message" onclick={link.callback(move |_| Msg::CopyTcpMsgClicked(index))} >{copy_icon}</div>
                        {time}
                        <div class="ws-open-arrow">{"❯"}</div>
                    </div>
                </label>

                <div class="atab-content">
                  {data.clone()}
                </div>
              </div>
            }
        }
    }
}

pub fn render_udp_msg(msg: &UdpMessage, link: &Scope<BoltApp>, index: usize) -> Html {
    let data = format!("{:?}", msg.data.clone());

    let txt = if data.len() > 60 {
        format!("{}...", &data[0..60])
    } else {
        data.clone()
    };

    let time = format_time(msg.timestamp);

    let copy_icon = crate::view::icons::copy_icon(20, 20);

    match msg.msg_type {
        UdpMsgType::IN => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-in-arrow">{"↓"}</div>
                        <div class="udp-msg-peer-address">{msg.peer_address.clone()}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
                        <div class="copy-msg-icon" title="copy message" onclick={link.callback(move |_| Msg::CopyUdpMsgClicked(index))} >{copy_icon}</div>
                        {time}
                        <div class="ws-open-arrow">{"❯"}</div>
                    </div>
                </label>

                <div class="atab-content">
                  {data.clone()}
                </div>
              </div>
            }
        }

        UdpMsgType::OUT => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-out-arrow">{"↑"}</div>
                        <div class="udp-msg-peer-address">{msg.peer_address.clone()}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
                        <div class="copy-msg-icon" title="copy message" onclick={link.callback(move |_| Msg::CopyUdpMsgClicked(index))} >{copy_icon}</div>
                        {time}
                        <div class="ws-open-arrow">{"❯"}</div>
                    </div>
                </label>

                <div class="atab-content">
                  {data.clone()}
                </div>
              </div>
            }
        }
    }
}

pub fn format_time(timestamp_ms: u64) -> String {
    let js = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp_ms as f64));
    let hour = js.get_hours() as u8;
    let minute = js.get_minutes() as u8;
    let second = js.get_seconds() as u8;

    format!("{:02}:{:02}:{:02}", hour, minute, second)
}
