use crate::view;
use crate::BoltContext;
use crate::Msg;
use crate::Page;
use bolt_common::prelude::*;
use yew::{html, AttrValue, Html};

pub fn http_response(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    let can_display = !bctx.http_requests.is_empty();

    let mut request = HttpRequest::new();

    if bctx.page == Page::HttpPage && can_display {
        request = bctx.http_requests[bctx.http_current].clone();
    }

    if bctx.page == Page::Collections && can_display {
        request = bctx.collections[bctx.col_current[0]].requests[bctx.col_current[1]].clone();
    }

    html! {
    <div class="resp">
        if can_display && !request.response.failed && !request.loading {
            <div class="respline">
                <div class="resptabs">
                    <div id="resp_body_tab" class={if request.resp_tab == 1  {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::HttpRespBodyPressed)}>{"Body"}</div>
                    <div id="resp_headers_tab" class={if request.resp_tab == 2  {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::HttpRespHeadersPressed)}>{"Headers"}</div>
                </div>

                <div class="respstats">
                    <div id="status" class="respstat">{"Status: "} {request.response.status}</div>
                    <div id="time" class="respstat">{"Time: "} {request.response.time} {" ms"}</div>
                    <div id="size" class="respstat">{"Size: "} {request.response.size} {" B"}</div>
                </div>
            </div>

            <div class="tabcontent">
                if request.resp_tab == 1 {
                    <div id="respbody" class="respbody" >
                        if request.response.response_type == HttpResponseType::JSON {
                            {Html::from_html_unchecked(AttrValue::from(request.response.body.clone()))}
                        } else {
                            {request.response.body.clone()}
                        }
                    </div>
                } else if request.resp_tab == 2 {
                    <div class="respheaders">
                        <table>
                            <tr>
                                <th>{"Header"}</th>
                                <th>{"Value"}</th>
                            </tr>
                            { for request.response.headers.iter().map(|header| view::header::render_http_resp_header(&header[0], &header[1])) }
                        </table>
                    </div>
                }
            </div>
        } else if can_display && request.loading {
            <div class="resploading"><img src="/icon/icon.png" /></div>
        } else if request.response.failed {
            <div class="resperror">{request.response.body.clone()}</div>
        }

    </div>
    }
}

// pub fn collection_response(bctx: &mut BoltContext) -> Html {
//     let link = bctx.link.as_ref().unwrap();

//     let can_display =
//         !bctx.collections.is_empty() && !bctx.collections[bctx.col_current[0]].requests.is_empty();

//     let mut request = HttpRequest::new();

//     if bctx.page == Page::HttpPage && can_display {
//         request = bctx.http_requests[bctx.http_current].clone();
//     }

//     if bctx.page == Page::Collections && can_display {
//         request = bctx.collections[bctx.col_current[0]].requests[bctx.col_current[1]].clone();
//     }

//     html! {
//     <div class="resp">
//         if can_display && !request.response.failed && !request.loading {
//             <div class="respline">
//                 <div class="resptabs">
//                     <div id="resp_body_tab" class={if request.resp_tab == 1  {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::RespBodyPressed)}>{"Body"}</div>
//                     <div id="resp_headers_tab" class={if request.resp_tab == 2  {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::RespHeadersPressed)}>{"Headers"}</div>
//                 </div>

//                 <div class="respstats">
//                     <div id="status" class="respstat">{"Status: "} {request.response.status}</div>
//                     <div id="time" class="respstat">{"Time: "} {request.response.time} {" ms"}</div>
//                     <div id="size" class="respstat">{"Size: "} {request.response.size} {" B"}</div>
//                 </div>
//             </div>

//             <div class="tabcontent">
//                 if request.resp_tab == 1 {
//                     <div id="respbody" class="respbody" >
//                         if request.response.response_type == HttpResponseType::JSON {
//                             {Html::from_html_unchecked(AttrValue::from(request.response.body.clone()))}
//                         } else {
//                             {request.response.body.clone()}
//                         }
//                     </div>
//                 } else if request.resp_tab == 2 {
//                     <div class="respheaders">
//                         <table>
//                             <tr>
//                                 <th>{"Header"}</th>
//                                 <th>{"Value"}</th>
//                             </tr>
//                             { for request.response.headers.iter().map(|header| view::header::render_header(&header[0], &header[1])) }
//                         </table>
//                     </div>
//                 }
//             </div>
//         } else if can_display && request.loading {
//             <div class="resploading"><img src="/icon/icon.png" /></div>
//         } else if request.response.failed {
//             <div class="resperror">{request.response.body.clone()}</div>
//         }

//     </div>
//     }
// }

pub fn ws_messages(bctx: &mut BoltContext) -> Html {
    // let link = bctx.link.as_ref().unwrap();

    let can_display = !bctx.ws_connections.is_empty();

    let mut connection = WsConnection::new();

    if can_display {
        connection = bctx.ws_connections[bctx.ws_current].clone();
    }

    html! {
        <div class="resp">
            if can_display && !connection.failed && !connection.loading {
                <div class="respline">
                    <div class="resptabs">
                        <div id="resp_body_tab" class={if connection.in_tab == 1  {"tab tabSelected"} else {"tab pointer"}}>{"Messages"}</div>
                    </div>

                    <div class="respstats">
                        if connection.connected {
                            <div id="status" class="respstat">{"Connected"}</div>
                        } else {
                            <div id="status" class="respstat">{"Disconnected"}</div>
                        }
                    </div>
                 </div>

                <div class="tabcontent">
                       // <div id="respbody" class="respbody" >
                       //      if request.response.response_type == HttpResponseType::JSON {
                       //          {Html::from_html_unchecked(AttrValue::from(request.response.body.clone()))}
                       //      } else {
                       //          {request.response.body.clone()}
                       //      }
                       //  </div>
                </div>
            } else if can_display && connection.loading {
                <div class="resploading"><img src="/icon/icon.png" /></div>
            } else if connection.failed {
                <div class="resperror">{"failed"}</div>
            }

        </div>
    }
}
