use bolt_common::prelude::*;
use yew::{html, Html};

pub fn render_ws_msg(msg: &WsMessage) -> Html {
    match msg.msg_type {
        WsMsgType::IN => {
            html! {
                <tr>
                    <td>{"IN"}</td>
                    <td>{msg.txt.clone()}</td>
                </tr>
            }
        }

        WsMsgType::OUT => {
            html! {
                <tr>
                    <td>{"OUT"}</td>
                    <td>{msg.txt.clone()}</td>
                </tr>
            }
        }
    }
}
