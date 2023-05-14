use bolt_common::prelude::*;
use yew::{html, Html};
use std::time::Duration;

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
                <div class="ws-msg pointer">
                    <div class="ws-msg-left">
                        <div class="ws-in-arrow">{"↓"}</div>
                        <div class="ws-msg-txt">{txt}</div>
                    </div>
                    <div>{time}</div>
                </div>
            }
        }

        WsMsgType::OUT => {
            html! {
                <div class="ws-msg pointer">
                    <div class="ws-msg-left">
                        <div class="ws-out-arrow">{"↑"}</div>
                        <div class="ws-msg-txt">{txt}</div>
                    </div>
                    <div>{time}</div>
                </div>
            }
        }
    }
}

fn format_time(timestamp: u64) -> String {
    let duration = Duration::from_millis(timestamp);
    let sec = duration.as_secs();
    
    let hours = sec / 3600 / 3600 / 30;
    let minutes = (sec % 3600) / 60;
    let seconds = sec % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
