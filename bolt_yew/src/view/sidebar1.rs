use crate::BoltContext;
use crate::Msg;
use crate::Page;
use yew::{html, Html};

use crate::view::icons;

pub fn sidebar(bctx: &mut BoltContext, page: Page) -> Html {
    let link = bctx.link.as_ref().unwrap();

    let http_icon = icons::http_icon(25, 25);
    let ws_icon = icons::websocket_icon(30, 30);
    let tcp_icon = icons::tcp_icon(25, 25);
    let udp_icon = icons::tcp_icon(25, 25);
    let servers_icon = icons::servers_icon(25, 25);
    let collections_icon = icons::collections_icon(25, 25);

    html! {
        <div class="sidebar1">
            <div class={if page == Page::HttpPage {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::HttpPage))}>
                {http_icon}
                {"HTTP"}
            </div>

           <div class={if page == Page::Websockets {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::Websockets))}>
                {ws_icon}
                {"Websocket"}
            </div>

           <div class={if page == Page::Tcp {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::Tcp))}>
                {tcp_icon}
                {"TCP"}
            </div>

           <div class={if page == Page::Udp {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::Udp))}>
                {udp_icon}
                {"UDP"}
           </div>

           <div class={if page == Page::Servers {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::Servers))}>
                {servers_icon}
                {"Servers"}
            </div>

            <div class={if page == Page::Collections {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::Collections) )}>
                {collections_icon}
                {"Collections"}
            </div>

            // <div class="sidebaritem pointer">
                // <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path fill="none" stroke-width="2" d="M8.9997,0.99999995 L8.9997,8.0003 L1.9997,20.0003 L1.9997,23.0003 L21.9997,23.0003 L21.9997,20.0003 L14.9997,8.0003 L14.9997,0.99999995 M15,18 C15.5522847,18 16,17.5522847 16,17 C16,16.4477153 15.5522847,16 15,16 C14.4477153,16 14,16.4477153 14,17 C14,17.5522847 14.4477153,18 15,18 Z M9,20 C9.55228475,20 10,19.5522847 10,19 C10,18.4477153 9.55228475,18 9,18 C8.44771525,18 8,18.4477153 8,19 C8,19.5522847 8.44771525,20 9,20 Z M18,13 C11,9.99999996 12,17.0000002 6,14 M5.9997,1.0003 L17.9997,1.0003"></path></svg>
                // {"Test"}
            // </div>
        </div>
    }
}
