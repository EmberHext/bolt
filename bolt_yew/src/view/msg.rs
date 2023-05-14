use bolt_common::prelude::*;
use yew::{html, Html};

pub fn render_ws_msg(msg: &WsMessage) -> Html {
    match msg.msg_type {
        WsMsgType::IN => {
            html! {
                <div class="ws-msg pointer">
                    <div class="ws-msg-left">
                        <div class="ws-in-arrow">{"↓"}</div>
                        <div class="ws-msg-txt">{msg.txt.clone()}</div>
                    </div>
                    <div>{"02:13:27"}</div>
                </div>
            }
        }

        WsMsgType::OUT => {
            html! {
                <div class="ws-msg pointer">
                    <div class="ws-msg-left">
                        <div class="ws-out-arrow">{"↑"}</div>
                        <div class="ws-msg-txt">{msg.txt.clone()}</div>
                    </div>
                    <div>{"02:13:27"}</div>
                </div>
            }
        }
    }
}
