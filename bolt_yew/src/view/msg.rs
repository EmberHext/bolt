use bolt_common::prelude::*;
use yew::{html, Html};

pub fn render_ws_msg(msg: &WsMessage) -> Html {
    let txt = msg.txt.clone();

    let txt = if txt.len() > 60 {
        format!("{}...", &txt[0..60])
    } else {
        txt
    };

    let time = format_time(msg.timestamp);

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

pub fn render_udp_msg(msg: &UdpMessage) -> Html {
    let txt = msg.txt.clone();

    let txt = if txt.len() > 60 {
        format!("{}...", &txt[0..60])
    } else {
        txt
    };

    let time = format_time(msg.timestamp);

    match msg.msg_type {
        UdpMsgType::IN => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-in-arrow">{"↓"}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
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

        UdpMsgType::OUT => {
            html! {
              <div class="atab">
                <input type="checkbox" id={msg.msg_id.clone()} />
                <label class="atab-label" for={msg.msg_id.clone()}>
                     <div class="ws-msg-left">
                        <div class="ws-out-arrow">{"↑"}</div>
                        <div class="ws-msg-txt">{txt}</div>
                     </div>

                    <div class="ws-msg-right">
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

pub fn format_time(timestamp_ms: u64) -> String {
    let js = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp_ms as f64));
    let hour = js.get_hours() as u8;
    let minute = js.get_minutes() as u8;
    let second = js.get_seconds() as u8;

    format!("{:02}:{:02}:{:02}", hour, minute, second)
}
