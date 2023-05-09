use crate::connect_ws;
use crate::send_http_request;
use crate::send_ws;
use crate::utils::*;
use crate::BoltContext;
use crate::Collection;
use crate::Msg;
use bolt_common::prelude::*;

pub fn process(bctx: &mut BoltContext, msg: Msg) -> bool {
    let should_render = match msg {
        Msg::Nothing => false,

        Msg::HttpReqSelectedMethod(meth) => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.method = meth;

            true
        }

        Msg::SendHttpPressed => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            send_http_request(current);

            true
        }

        Msg::ConnectWsPressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            connect_ws(current);

            true
        }

        Msg::SendWsPressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            send_ws(current);

            true
        }

        
        Msg::HelpPressed => {
            open_link("https://github.com/hiro-codes/bolt/tree/master/docs".to_string());

            true
        }

        Msg::GithubPressed => {
            open_link("https://github.com/hiro-codes/bolt".to_string());

            true
        }

        Msg::HttpReqBodyPressed => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.req_tab = 1;

            true
        }

        Msg::HttpReqHeadersPressed => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.req_tab = 3;

            true
        }

        Msg::HttpReqParamsPressed => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.req_tab = 2;

            true
        }

        Msg::WsOutMessagePressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];
            current.out_tab = 1;

            true
        }

        Msg::WsOutHeadersPressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];
            current.out_tab = 3;

            true
        }

        Msg::WsOutParamsPressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];
            current.out_tab = 2;

            true
        }

        Msg::HttpRespBodyPressed => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.resp_tab = 1;

            true
        }

        Msg::HttpRespHeadersPressed => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.resp_tab = 2;

            true
        }

        Msg::HttpReceivedResponse => true,

        Msg::HttpReqAddHeader => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.headers.push(vec!["".to_string(), "".to_string()]);

            true
        }

        Msg::HttpReqRemoveHeader(index) => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.headers.remove(index);

            true
        }

        Msg::WsOutAddHeader => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            current.out_headers.push(vec!["".to_string(), "".to_string()]);

            true
        }

        Msg::WsOutRemoveHeader(index) => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            current.out_headers.remove(index);

            true
        }

        Msg::HttpReqAddParam => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.params.push(vec!["".to_string(), "".to_string()]);
            true
        }


        Msg::HttpReqRemoveParam(index) => {
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.params.remove(index);
            true
        }


        Msg::WsOutAddParam => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            current.out_params.push(vec!["".to_string(), "".to_string()]);
            true
        }


        Msg::WsOutRemoveParam(index) => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            current.out_params.remove(index);
            true
        }

        Msg::AddCollection => {
            let mut new_collection = Collection::new();

            new_collection.name = new_collection.name + &(bctx.main_state.collections.len() + 1).to_string();
            bctx.main_state.collections.push(new_collection);

            true
        }

        Msg::RemoveCollection(index) => {
            bctx.main_state.collections.remove(index);

            bctx.main_state.col_current = vec![0, 0];

            true
        }

        Msg::HttpReqMethodChanged => {
            let method = get_method();

            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.method = method;
            true
        }

        Msg::UrlChanged => {
            let url = get_url();

            if bctx.main_state.page == Page::HttpPage {
                let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

                current.url = url.clone();
                current.name = url;
            } else if bctx.main_state.page == Page::Websockets {
                let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

                current.url = url.clone();
                current.name = url;
            }

            true
        }

        Msg::HttpReqBodyChanged => {
            let body = get_body();
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.body = body;

            true
        }

        Msg::WsOutMessageChanged => {
            let message = get_body();
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];
            current.out_message = message;

            true
        }

        Msg::HttpReqHeaderChanged(index) => {
            let header = get_header(index);

            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.headers[index] = header;

            true
        }

        Msg::WsOutHeaderChanged(index) => {
            let header = get_header(index);

            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            current.out_headers[index] = header;

            true
        }

        Msg::HttpReqParamChanged(index) => {
            let param = get_param(index);

            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.params[index] = param;

            true
        }

        Msg::WsOutParamChanged(index) => {
            let param = get_param(index);

            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            current.out_params[index] = param;

            true
        }

        Msg::AddHttpRequest => {
            let mut new_request = HttpRequest::new();
            new_request.name = new_request.name + &(bctx.main_state.http_requests.len() + 1).to_string();

            bctx.main_state.http_requests.push(new_request);

            true
        }

        Msg::AddWsRequest => {
            let mut new_request = WsConnection::new();
            new_request.name = new_request.name + &(bctx.main_state.ws_connections.len() + 1).to_string();

            bctx.main_state.ws_connections.push(new_request);

            true
        }

        Msg::AddToCollection(index) => {
            let collection = &mut bctx.main_state.collections[index];

            let mut new_request = HttpRequest::new();
            new_request.name = new_request.name + &(collection.requests.len() + 1).to_string();

            collection.requests.push(new_request);

            true
        }

        Msg::ToggleCollapsed(index) => {
            let collection = &mut bctx.main_state.collections[index];

            collection.collapsed = !collection.collapsed;

            true
        }

        Msg::RemoveHttpRequest(index) => {
            bctx.main_state.http_requests.remove(index);
            if !bctx.main_state.http_requests.is_empty() && bctx.main_state.http_current > bctx.main_state.http_requests.len() - 1 {
                bctx.main_state.http_current = bctx.main_state.http_requests.len() - 1;
            }

            true
        }

        Msg::RemoveWsRequest(index) => {
            bctx.main_state.ws_connections.remove(index);
            if !bctx.main_state.ws_connections.is_empty() && bctx.main_state.ws_current > bctx.main_state.ws_connections.len() - 1 {
                bctx.main_state.ws_current = bctx.main_state.ws_connections.len() - 1;
            }

            true
        }

        Msg::RemoveFromCollection(col_index, req_index) => {
            bctx.main_state.collections[col_index].requests.remove(req_index);
            bctx.main_state.col_current = vec![0, 0];

            true
        }

        Msg::SelectHttpRequest(index) => {
            let mut new_index = index;

            if bctx.main_state.http_requests.len() == 0 {
                bctx.main_state.http_current = new_index;

                // bctx.main_state.main_col.requests[new_index].response.request_index = new_index;
            } else {
                if index >= bctx.main_state.http_requests.len() {
                    new_index = bctx.main_state.http_requests.len() - 1;
                    bctx.main_state.http_current = new_index;

                    bctx.main_state.http_requests[new_index].response.request_index = new_index;
                } else {
                    bctx.main_state.http_current = new_index;

                    bctx.main_state.http_requests[new_index].response.request_index = new_index;
                }
            }

            true
        }

        Msg::SelectWsRequest(index) => {
            let mut new_index = index;

            if bctx.main_state.ws_connections.len() == 0 {
                bctx.main_state.ws_current = new_index;

                // bctx.main_state.main_col.requests[new_index].response.request_index = new_index;
            } else {
                if index >= bctx.main_state.ws_connections.len() {
                    new_index = bctx.main_state.ws_connections.len() - 1;
                    bctx.main_state.ws_current = new_index;
                } else {
                    bctx.main_state.ws_current = new_index;
                }
            }

            true
        }

        Msg::SelectFromCollection(col_index, req_index) => {
            bctx.main_state.col_current = vec![col_index, req_index];

            bctx.main_state.collections[col_index].requests[req_index]
                .response
                .request_index = req_index;

            true
        }

        Msg::Update => true,

        Msg::SwitchPage(page) => {
            bctx.main_state.page = page;

            true
        }
    };

    should_render
}
