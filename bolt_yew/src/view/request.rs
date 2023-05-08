use crate::helpers::enums::HttpReqTabs;
use crate::helpers::enums::WsOutTabs;
use crate::view;
use crate::BoltContext;
use crate::Msg;
use yew::KeyboardEvent;
use yew::{html, Html};

use bolt_common::prelude::*;

pub fn http_request(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    let can_display = !bctx.http_requests.is_empty();

    let mut request = HttpRequest::new();

    if can_display {
        request = bctx.http_requests[bctx.http_current].clone()
    }

    let selected_method = request.method.to_string();

    html! {
        <div class="req">
        if can_display {
            <div class="requestbar">
                <div class="">
                    <select id="methodselect" class="methodselect pointer" onchange={link.callback(|_| Msg::HttpReqMethodChanged)}>
                        { for (0..HttpMethod::count()).map(|index| {
                            let current_method_option: HttpMethod = HttpMethod::from(index);
                            let value = current_method_option.to_string().to_lowercase();
                            html! {
                                <option value={value.clone()} selected={is_selected(&selected_method, &value)}>{current_method_option}</option>
                            }
                        })}
                    </select>
                </div>

                <input id="urlinput" class="urlinput" type="text" autocomplete="off" spellcheck="false" value={request.url.clone()} placeholder="http://" onkeydown={link.callback(|e: KeyboardEvent| { if e.key() == "Enter" { Msg::SendHttpPressed } else { Msg::Nothing } })}  oninput={link.callback(|_|{ Msg::UrlChanged })} />

                <button class="sendbtn pointer" type="button" onclick={link.callback(|_| Msg::SendHttpPressed)}>{"Send"}</button>
            </div>

            <div class="reqtabs">
                <div id="req_body_tab" class={if is_tab_selected(&request.req_tab, HttpReqTabs::Body) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::HttpReqBodyPressed)}>{"Body"}</div>
                <div id="req_params_tab" class={if is_tab_selected(&request.req_tab, HttpReqTabs::Params) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::HttpReqParamsPressed)}>{"Params"}</div>
                <div id="req_headers_tab" class={if is_tab_selected(&request.req_tab, HttpReqTabs::Headers) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::HttpReqHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="tabcontent">
                if is_tab_selected(&request.req_tab, HttpReqTabs::Body) {
                    <textarea autocomplete="off" spellcheck="false" id="reqbody" class="reqbody" value={request.body.clone()} placeholder="Request body" onchange={link.callback(|_| Msg::HttpReqBodyChanged)}>

                    </textarea>
                } else if is_tab_selected(&request.req_tab, HttpReqTabs::Params) {
                    <div class="reqheaders">
                        <table>
                            <tr>
                                <th>{"Key"}</th>
                                <th>{"Value"}</th>
                            </tr>
                            { for request.params.iter().enumerate().map(|(index, header)| view::param::render_http_req_params(bctx, index, request.params.len(), &header[0], &header[1])) }
                        </table>
                    </div>

                } else if is_tab_selected(&request.req_tab, HttpReqTabs::Headers) {
                    <div class="reqheaders">
                        <table>
                            <tr>
                                <th>{"Header"}</th>
                                <th>{"Value"}</th>
                            </tr>
                            { for request.headers.iter().enumerate().map(|(index, header)| view::header::render_http_req_header(bctx, index, request.headers.len(), &header[0], &header[1])) }
                        </table>
                    </div>
                }
            </div>
        }
        </div>

    }
}

fn is_selected(method: &str, option_value: &str) -> bool {
    method.to_lowercase() == option_value.to_lowercase()
}


fn is_ws_tab_selected(request_tab: &u8, tab: WsOutTabs) -> bool {
    *request_tab == u8::from(tab)
}

fn is_tab_selected(request_tab: &u8, tab: HttpReqTabs) -> bool {
    *request_tab == u8::from(tab)
}

pub fn ws_connection(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    let can_display = !bctx.ws_connections.is_empty();

    let mut connection = WsConnection::new();

    if can_display {
        connection = bctx.ws_connections[bctx.ws_current].clone();
    }

    html! {
        <div class="req">
        if can_display {
            <div class="requestbar">
                <input id="urlinput" class="urlinput" type="text" autocomplete="off" spellcheck="false" value={connection.url.clone()} placeholder="ws://" onkeydown={link.callback(|e: KeyboardEvent| { if e.key() == "Enter" { Msg::ConnectWsPressed } else { Msg::Nothing } })}  oninput={link.callback(|_|{ Msg::UrlChanged })} />

                <button class="sendbtn pointer" type="button" onclick={link.callback(|_| Msg::ConnectWsPressed)}>{"Connect"}</button>
            </div>

            <div class="reqtabs">
                <div id="req_body_tab" class={if is_ws_tab_selected(&connection.out_tab, WsOutTabs::Message) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::WsOutMessagePressed)}>{"Message"}</div>
                <div id="req_params_tab" class={if is_ws_tab_selected(&connection.out_tab, WsOutTabs::Params) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::WsOutParamsPressed)}>{"Params"}</div>
                <div id="req_headers_tab" class={if is_ws_tab_selected(&connection.out_tab, WsOutTabs::Headers) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::WsOutHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="tabcontent">
                if is_ws_tab_selected(&connection.out_tab, WsOutTabs::Message) {
                    <textarea autocomplete="off" spellcheck="false" id="reqbody" class="reqbody" value={connection.out_message.clone()} placeholder="Compose Message" onchange={link.callback(|_| Msg::WsOutMessageChanged)}>

                    </textarea>
                } else if is_ws_tab_selected(&connection.out_tab, WsOutTabs::Params) {
                    <div class="reqheaders">
                        <table>
                            <tr>
                                <th>{"Key"}</th>
                                <th>{"Value"}</th>
                            </tr>
                            { for connection.out_params.iter().enumerate().map(|(index, header)| view::param::render_ws_out_params(bctx, index, connection.out_params.len(), &header[0], &header[1])) }
                        </table>
                    </div>

                } else if is_ws_tab_selected(&connection.out_tab, WsOutTabs::Headers) {
                    <div class="reqheaders">
                        <table>
                            <tr>
                                <th>{"Header"}</th>
                                <th>{"Value"}</th>
                            </tr>
                            { for connection.out_headers.iter().enumerate().map(|(index, header)| view::header::render_ws_out_header(bctx, index, connection.out_headers.len(), &header[0], &header[1])) }
                        </table>
                    </div>
                }
            </div>
        }
        </div>

    }
}

// pub fn collection_request(bctx: &mut BoltContext) -> Html {
//     let link = bctx.link.as_ref().unwrap();

//     let can_display =
//         !bctx.collections.is_empty() && !bctx.collections[bctx.col_current[0]].requests.is_empty();

//     let mut request = HttpRequest::new();

//     if can_display {
//         request = bctx.collections[bctx.col_current[0]].requests[bctx.col_current[1]].clone()
//     }

//     let selected_method = request.method.to_string();

//     html! {
//         <div class="req">
//         if can_display {
//             <div class="requestbar">
//                 <div class="">
//                     <select id="methodselect" class="methodselect pointer" onchange={link.callback(|_| Msg::MethodChanged)}>
//                         { for (0..HttpMethod::count()).map(|index| {
//                             let current_method_option: HttpMethod = HttpMethod::from(index);
//                             let value = current_method_option.to_string().to_lowercase();
//                             html! {
//                                 <option value={value.clone()} selected={is_selected(&selected_method, &value)}>{current_method_option}</option>
//                             }
//                         })}
//                     </select>
//                 </div>

//                 <input id="urlinput" class="urlinput" type="text" autocomplete="off" spellcheck="false" value={request.url.clone()} placeholder="http://" onkeydown={link.callback(|e: KeyboardEvent| { if e.key() == "Enter" { Msg::SendPressed } else { Msg::Nothing } })}  oninput={link.callback(|_|{ Msg::UrlChanged })} />

//                 <button class="sendbtn pointer" type="button" onclick={link.callback(|_| Msg::SendPressed)}>{"Send"}</button>
//             </div>

//             <div class="reqtabs">
//                 <div id="req_body_tab" class={if is_tab_selected(&request.req_tab, Body) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqBodyPressed)}>{"Body"}</div>
//                 <div id="req_params_tab" class={if is_tab_selected(&request.req_tab, Params) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqParamsPressed)}>{"Params"}</div>
//                 <div id="req_headers_tab" class={if is_tab_selected(&request.req_tab, Headers) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqHeadersPressed)}>{"Headers"}</div>
//             </div>

//             <div class="tabcontent">
//                 if is_tab_selected(&request.req_tab, Body) {
//                     <textarea autocomplete="off" spellcheck="false" id="reqbody" class="reqbody" value={request.body.clone()} placeholder="Request body" onchange={link.callback(|_| Msg::BodyChanged)}>

//                     </textarea>
//                 } else if is_tab_selected(&request.req_tab, Params) {
//                     <div class="reqheaders">
//                         <table>
//                             <tr>
//                                 <th>{"Key"}</th>
//                                 <th>{"Value"}</th>
//                             </tr>
//                             { for request.params.iter().enumerate().map(|(index, header)| view::param::render_params(bctx, index, request.params.len(), &header[0], &header[1])) }
//                         </table>
//                     </div>

//                 } else if is_tab_selected(&request.req_tab, Headers) {
//                     <div class="reqheaders">
//                         <table>
//                             <tr>
//                                 <th>{"Header"}</th>
//                                 <th>{"Value"}</th>
//                             </tr>
//                             { for request.headers.iter().enumerate().map(|(index, header)| view::header::render_reqheader(bctx, index, request.headers.len(), &header[0], &header[1])) }
//                         </table>
//                     </div>
//                 }
//             </div>
//         }
//         </div>

//     }
// }
