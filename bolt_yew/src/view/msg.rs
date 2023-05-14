use bolt_common::prelude::*;
use yew::{html, Html};

pub fn render_ws_msg(msg: &WsMessage) -> Html {
    match msg.msg_type {
        WsMsgType::IN => {
            html! {
                <div>
                    <div>{"IN"}</div>
                    <div>{msg.txt.clone()}</div>
                </div>
            }
        }

        WsMsgType::OUT => {
            html! {
                <div>
                    <div>{"OUT"}</div>
                    <div>{msg.txt.clone()}</div>
                </div>
            }
        }
    }
}
