use crate::BoltApp;
use yew::html::Scope;

use crate::BoltContext;
use crate::Collection;
use crate::Msg;
use bolt_common::prelude::*;
use yew::{html, Html};

pub fn sidebar_http(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddHttpRequest)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.main_state.http_requests.iter().enumerate().map(|(index, req)| render_http_request(bctx.link.as_ref().unwrap(), bctx.main_state.http_current, index, req))}

        </div>
    }
}

pub fn sidebar_websockets(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddWsConnection)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.main_state.ws_connections.iter().enumerate().map(|(index, req)| render_ws_connection(bctx.link.as_ref().unwrap(), bctx.main_state.ws_current, index, req))}

        </div>
    }
}

pub fn sidebar_servers(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddHttpRequest)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.main_state.http_requests.iter().enumerate().map(|(index, req)| render_http_request(bctx.link.as_ref().unwrap(), bctx.main_state.http_current, index, req))}

        </div>
    }
}

pub fn sidebar_tcp(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddTcpConnection)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.main_state.tcp_connections.iter().enumerate().map(|(index, req)| render_tcp_connection(bctx.link.as_ref().unwrap(), bctx.main_state.tcp_current, index, req))}

        </div>
    }
}

pub fn sidebar_udp(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddUdpConnection)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.main_state.udp_connections.iter().enumerate().map(|(index, req)| render_udp_connection(bctx.link.as_ref().unwrap(), bctx.main_state.udp_current, index, req))}

        </div>
    }
}

pub fn sidebar_collections(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddCollection)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.main_state.collections.iter().enumerate().map(|(index, col)| render_collection(&mut bctx.link.as_ref().unwrap(), index, bctx.main_state.col_current.clone(), col))}

        </div>
    }
}

fn render_collection(
    link: &Scope<BoltApp>,
    index: usize,
    current: Vec<usize>,
    col: &Collection,
) -> Html {
    html! {
        <>
        <div id={"request".to_string() + &index.to_string()} class="sidebar2item">

            if col.collapsed {
                <div onclick={link.callback(move |_| Msg::ToggleCollapsed(index))} class="col-arrow pointer">{">"}</div>
            } else {
                <div onclick={link.callback(move |_| Msg::ToggleCollapsed(index))} class="col-arrow pointer">{"⌄"}</div>
            }

            <div>{col.name.clone()}</div>

            <div class="col-icons">
            <div class="pointer add-col" onclick={link.callback(move |_| Msg::AddToCollection(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="15px" width="15px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
            </div>

            <div class="pointer bin-col" onclick={link.callback(move |_| Msg::RemoveCollection(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="15px" width="15px"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
            </div>
        </div>
        if !col.collapsed {
            { for col.requests.iter().enumerate().map(|(req_index, req)| render_col_request(link, req_index, index, current.clone(), req))}
        }

        </>
    }
}

fn render_http_request(
    link: &Scope<BoltApp>,
    current: usize,
    index: usize,
    req: &HttpRequest,
) -> Html {
    let request_name = req.name.clone();

    let request_name = if request_name.len() > 20 {
        format!("{}...", &request_name[0..20])
    } else {
        request_name
    };

    html! {
        <div onclick={link.callback(move |_| Msg::SelectHttpRequest(index))} id={"request".to_string() + &index.to_string()} class={if index == current { "pointer sidebar2item sidebar2item-selected" } else { "pointer sidebar2item" }} >
            <div class="requestname">{request_name}</div>
            <div class="pointer bin-req" title="delete" onclick={link.callback(move |_| Msg::RemoveHttpRequest(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
        </div>
    }
}

fn render_tcp_connection(
    link: &Scope<BoltApp>,
    current: usize,
    index: usize,
    req: &TcpConnection,
) -> Html {
    let request_name = req.name.clone();

    let request_name = if request_name.len() > 20 {
        format!("{}...", &request_name[0..20])
    } else {
        request_name
    };

    html! {
        <div onclick={link.callback(move |_| Msg::SelectTcpConnection(index))} id={"request".to_string() + &index.to_string()} class={if index == current { "pointer sidebar2item sidebar2item-selected" } else { "pointer sidebar2item" }} >
            <div class="requestname">{request_name}</div>
            <div class="pointer bin-req" title="delete" onclick={link.callback(move |_| Msg::RemoveTcpConnection(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
        </div>
    }
}

fn render_udp_connection(
    link: &Scope<BoltApp>,
    current: usize,
    index: usize,
    req: &UdpConnection,
) -> Html {
    let request_name = req.name.clone();

    let request_name = if request_name.len() > 20 {
        format!("{}...", &request_name[0..20])
    } else {
        request_name
    };

    html! {
        <div onclick={link.callback(move |_| Msg::SelectUdpConnection(index))} id={"request".to_string() + &index.to_string()} class={if index == current { "pointer sidebar2item sidebar2item-selected" } else { "pointer sidebar2item" }} >
            <div class="requestname">{request_name}</div>
            <div class="pointer bin-req" title="delete" onclick={link.callback(move |_| Msg::RemoveUdpConnection(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
        </div>
    }
}

fn render_ws_connection(
    link: &Scope<BoltApp>,
    current: usize,
    index: usize,
    req: &WsConnection,
) -> Html {
    let request_name = req.name.clone();
    // let request_name = req.connection_id.clone();

    let request_name = if request_name.len() > 20 {
        format!("{}...", &request_name[0..20])
    } else {
        request_name
    };

    html! {
        <div onclick={link.callback(move |_| Msg::SelectWsConnection(index))} id={"request".to_string() + &index.to_string()} class={if index == current { "pointer sidebar2item sidebar2item-selected" } else { "pointer sidebar2item" }} >
            <div class="requestname">{request_name}</div>
            <div class="pointer bin-req" title="delete" onclick={link.callback(move |_| Msg::RemoveWsConnection(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
        </div>
    }
}

fn render_col_request(
    link: &Scope<BoltApp>,
    req_index: usize,
    col_index: usize,
    current: Vec<usize>,
    req: &HttpRequest,
) -> Html {
    html! {
        <div id={"request".to_string() + &req_index.to_string()} class={if col_index == current[0] && req_index == current[1] { "sidebar2item-child sidebar2item-selected" } else { "sidebar2item-child" }} >
            <div class="pointer" onclick={link.callback(move |_| Msg::SelectFromCollection(col_index, req_index))}>{req.name.clone()}</div>
            <div class="pointer bin-req" onclick={link.callback(move |_| Msg::RemoveFromCollection(col_index, req_index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
        </div>
    }
}
