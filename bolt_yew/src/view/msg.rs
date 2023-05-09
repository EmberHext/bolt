use bolt_common::prelude::*;
use yew::{html, Html};

pub fn render_ws_msg(msg: &WsMessage) -> Html {
    html! {
        <tr>
            <td>{"IN"}</td>
            <td>{msg.txt.clone()}</td>
        </tr>
    }
}
