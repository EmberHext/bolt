// use bolt_ws::prelude::HttpMethod;

pub enum HttpReqTabs {
    Body,
    Params,
    Headers,
}

impl From<u8> for HttpReqTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => HttpReqTabs::Body,
            2 => HttpReqTabs::Params,
            3 => HttpReqTabs::Headers,
            _ => panic!("Invalid value for HttpReqTabs"),
        }
    }
}

impl From<HttpReqTabs> for u8 {
    fn from(tab: HttpReqTabs) -> Self {
        match tab {
            HttpReqTabs::Body => 1,
            HttpReqTabs::Params => 2,
            HttpReqTabs::Headers => 3,
        }
    }
}

pub enum HttpRespTabs {
    Body,
    Headers,
}

impl From<u8> for HttpRespTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => HttpRespTabs::Body,
            2 => HttpRespTabs::Headers,
            _ => panic!("Invalid value for HttpRespTabs"),
        }
    }
}

impl From<HttpRespTabs> for u8 {
    fn from(tab: HttpRespTabs) -> Self {
        match tab {
            HttpRespTabs::Body => 1,
            HttpRespTabs::Headers => 2,
        }
    }
}

pub enum WsOutTabs {
    Message,
    Params,
    Headers,
}

impl From<u8> for WsOutTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => WsOutTabs::Message,
            2 => WsOutTabs::Params,
            3 => WsOutTabs::Headers,
            _ => panic!("Invalid value for WsOutTabs"),
        }
    }
}

impl From<WsOutTabs> for u8 {
    fn from(tab: WsOutTabs) -> Self {
        match tab {
            WsOutTabs::Message => 1,
            WsOutTabs::Params => 2,
            WsOutTabs::Headers => 3,
        }
    }
}

pub enum WsInTabs {
    Messages,
}

impl From<u8> for WsInTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => WsInTabs::Messages,
            _ => panic!("Invalid value for WsInTabs"),
        }
    }
}

impl From<WsInTabs> for u8 {
    fn from(tab: WsInTabs) -> Self {
        match tab {
            WsInTabs::Messages => 1,
        }
    }
}
