use crate::connect_tcp;
use crate::connect_udp;
use crate::connect_ws;
use crate::disconnect_tcp;
use crate::disconnect_udp;
use crate::disconnect_ws;
use crate::send_http_request;
use crate::send_tcp;
use crate::send_udp;
use crate::send_ws;
use crate::utils::*;
use crate::BoltContext;
use crate::Collection;
use crate::Msg;
use bolt_common::prelude::*;

pub fn process(bctx: &mut BoltContext, msg: Msg) -> bool {
    let should_render = match msg {
        Msg::Nothing => false,

        // HTTP -------------------------------------------------------------
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
        Msg::HttpReqMethodChanged => {
            let method = get_method();

            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.method = method;
            true
        }
        Msg::HttpReqBodyChanged => {
            let body = get_body();
            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];
            current.body = body;

            true
        }
        Msg::HttpReqHeaderChanged(index) => {
            let header = get_header(index);

            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.headers[index] = header;

            true
        }
        Msg::HttpReqParamChanged(index) => {
            let param = get_param(index);

            let current = &mut bctx.main_state.http_requests[bctx.main_state.http_current];

            current.params[index] = param;

            true
        }
        Msg::AddHttpRequest => {
            let mut new_request = HttpRequest::new();
            new_request.name =
                new_request.name + &(bctx.main_state.http_requests.len() + 1).to_string();

            bctx.main_state.http_requests.push(new_request);

            true
        }
        Msg::RemoveHttpRequest(index) => {
            bctx.main_state.http_requests.remove(index);
            if !bctx.main_state.http_requests.is_empty()
                && bctx.main_state.http_current > bctx.main_state.http_requests.len() - 1
            {
                bctx.main_state.http_current = bctx.main_state.http_requests.len() - 1;
            }

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

                    bctx.main_state.http_requests[new_index]
                        .response
                        .request_index = new_index;
                } else {
                    bctx.main_state.http_current = new_index;

                    bctx.main_state.http_requests[new_index]
                        .response
                        .request_index = new_index;
                }
            }

            true
        }

        // WEBSOCKETS-------------------------------------------------------------
        Msg::WsOutMessageChanged => {
            let message = get_body();
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];
            current.out_buffer = message;

            true
        }
        Msg::AddWsConnection => {
            let mut new_request = WsConnection::new();
            new_request.name =
                new_request.name + &(bctx.main_state.ws_connections.len() + 1).to_string();

            let msg = AddWsConnectionMsg {
                msg_type: MsgType::ADD_WS_CONNECTION,
                connection_id: new_request.connection_id.clone(),
            };

            let msg = serde_json::to_string(&msg).unwrap();

            ws_write(msg);

            bctx.main_state.ws_connections.push(new_request);

            true
        }
        Msg::RemoveWsConnection(index) => {
            bctx.main_state.ws_connections.remove(index);
            if !bctx.main_state.ws_connections.is_empty()
                && bctx.main_state.ws_current > bctx.main_state.ws_connections.len() - 1
            {
                bctx.main_state.ws_current = bctx.main_state.ws_connections.len() - 1;
            }

            true
        }
        Msg::SelectWsConnection(index) => {
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
        Msg::WsOutMessagePressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];
            current.out_tab = 1;

            true
        }
        Msg::ConnectWsPressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            connect_ws(current);

            true
        }
        Msg::DisconnectWsPressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            disconnect_ws(current);

            true
        }
        Msg::SendWsPressed => {
            let current = &mut bctx.main_state.ws_connections[bctx.main_state.ws_current];

            send_ws(current);

            true
        }

        // TCP-------------------------------------------------------------
        Msg::TcpOutMessagePressed => {
            let current = &mut bctx.main_state.tcp_connections[bctx.main_state.tcp_current];
            current.out_tab = 1;

            true
        }
        Msg::TcpPeerUrlChanged => {
            let url = get_tcp_peer_url();

            let current = &mut bctx.main_state.tcp_connections[bctx.main_state.tcp_current];

            current.peer_address = url.clone();

            true
        }
        Msg::TcpOutMessageChanged => {
            let data = get_tcp_out_txt();
            let current = &mut bctx.main_state.tcp_connections[bctx.main_state.tcp_current];
            current.out_data_buffer = data;

            true
        }
        Msg::RemoveTcpConnection(index) => {
            bctx.main_state.tcp_connections.remove(index);
            if !bctx.main_state.tcp_connections.is_empty()
                && bctx.main_state.tcp_current > bctx.main_state.tcp_connections.len() - 1
            {
                bctx.main_state.tcp_current = bctx.main_state.tcp_connections.len() - 1;
            }

            true
        }
        Msg::SelectTcpConnection(index) => {
            let mut new_index = index;

            if bctx.main_state.tcp_connections.len() == 0 {
                bctx.main_state.tcp_current = new_index;

                // bctx.main_state.main_col.requests[new_index].response.request_index = new_index;
            } else {
                if index >= bctx.main_state.tcp_connections.len() {
                    new_index = bctx.main_state.tcp_connections.len() - 1;
                    bctx.main_state.tcp_current = new_index;
                } else {
                    bctx.main_state.tcp_current = new_index;
                }
            }

            true
        }
        Msg::ConnectTcpPressed => {
            let current = &mut bctx.main_state.tcp_connections[bctx.main_state.tcp_current];

            connect_tcp(current);

            true
        }
        Msg::DisconnectTcpPressed => {
            let current = &mut bctx.main_state.tcp_connections[bctx.main_state.tcp_current];

            disconnect_tcp(current);

            true
        }
        Msg::SendTcpPressed => {
            let current = &mut bctx.main_state.tcp_connections[bctx.main_state.tcp_current];

            send_tcp(current);

            true
        }
        Msg::AddTcpConnection => {
            let mut new_connection = TcpConnection::new();

            new_connection.name =
                new_connection.name + &(bctx.main_state.tcp_connections.len() + 1).to_string();

            let msg = AddTcpConnectionMsg {
                msg_type: MsgType::ADD_TCP_CONNECTION,
                connection_id: new_connection.connection_id.clone(),
            };

            let msg = serde_json::to_string(&msg).unwrap();

            ws_write(msg);

            bctx.main_state.tcp_connections.push(new_connection);

            true
        }

        // UDP-------------------------------------------------------------
        Msg::UdpOutMessagePressed => {
            let current = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];
            current.out_tab = 1;

            true
        }
        Msg::UdpPeerUrlChanged => {
            let url = get_udp_peer_url();

            let current = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];

            current.peer_address = url.clone();

            true
        }
        Msg::UdpOutMessageChanged => {
            let data = get_udp_out_txt();
            let current = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];
            current.out_data_buffer = data;

            true
        }
        Msg::RemoveUdpConnection(index) => {
            bctx.main_state.udp_connections.remove(index);
            if !bctx.main_state.udp_connections.is_empty()
                && bctx.main_state.udp_current > bctx.main_state.udp_connections.len() - 1
            {
                bctx.main_state.udp_current = bctx.main_state.udp_connections.len() - 1;
            }

            true
        }
        Msg::SelectUdpConnection(index) => {
            let mut new_index = index;

            if bctx.main_state.udp_connections.len() == 0 {
                bctx.main_state.udp_current = new_index;

                // bctx.main_state.main_col.requests[new_index].response.request_index = new_index;
            } else {
                if index >= bctx.main_state.udp_connections.len() {
                    new_index = bctx.main_state.udp_connections.len() - 1;
                    bctx.main_state.udp_current = new_index;
                } else {
                    bctx.main_state.udp_current = new_index;
                }
            }

            true
        }
        Msg::ConnectUdpPressed => {
            let current = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];

            connect_udp(current);

            true
        }
        Msg::DisconnectUdpPressed => {
            let current = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];

            disconnect_udp(current);

            true
        }
        Msg::SendUdpPressed => {
            let current = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];

            send_udp(current);

            true
        }
        Msg::AddUdpConnection => {
            let mut new_connection = UdpConnection::new();

            new_connection.name =
                new_connection.name + &(bctx.main_state.udp_connections.len() + 1).to_string();

            let msg = AddUdpConnectionMsg {
                msg_type: MsgType::ADD_UDP_CONNECTION,
                connection_id: new_connection.connection_id.clone(),
            };

            let msg = serde_json::to_string(&msg).unwrap();

            ws_write(msg);

            bctx.main_state.udp_connections.push(new_connection);

            true
        }

        // COLLECTIONS-------------------------------------------------------------
        Msg::AddCollection => {
            let mut new_collection = Collection::new();

            new_collection.name =
                new_collection.name + &(bctx.main_state.collections.len() + 1).to_string();
            bctx.main_state.collections.push(new_collection);

            true
        }
        Msg::RemoveCollection(index) => {
            bctx.main_state.collections.remove(index);

            bctx.main_state.col_current = vec![0, 0];

            true
        }
        Msg::AddToCollection(index) => {
            let collection = &mut bctx.main_state.collections[index];

            let mut new_request = HttpRequest::new();
            new_request.name = new_request.name + &(collection.requests.len() + 1).to_string();

            collection.requests.push(new_request);

            true
        }
        Msg::RemoveFromCollection(col_index, req_index) => {
            bctx.main_state.collections[col_index]
                .requests
                .remove(req_index);
            bctx.main_state.col_current = vec![0, 0];

            true
        }
        Msg::SelectFromCollection(col_index, req_index) => {
            bctx.main_state.col_current = vec![col_index, req_index];

            bctx.main_state.collections[col_index].requests[req_index]
                .response
                .request_index = req_index;

            true
        }

        // OTHER-------------------------------------------------------------
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
            } else if bctx.main_state.page == Page::Udp {
                let current = &mut bctx.main_state.udp_connections[bctx.main_state.udp_current];

                current.host_address = url.clone();
            }

            true
        }
        Msg::ToggleCollapsed(index) => {
            let collection = &mut bctx.main_state.collections[index];

            collection.collapsed = !collection.collapsed;

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
        Msg::Update => true,
        Msg::SwitchPage(page) => {
            bctx.main_state.page = page;

            true
        }
    };

    should_render
}
