use serde::{Deserialize, Serialize};
use crate::http::HttpRequest;

#[derive(Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<HttpRequest>,
    pub collapsed: bool,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            name: "New Collection ".to_string(),
            requests: vec![],
            collapsed: false,
        }
    }
}
