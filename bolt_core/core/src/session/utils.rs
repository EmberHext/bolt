use bolt_common::prelude::*;
use std::time::SystemTime;

pub fn get_home() -> String {
    let path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/bolt/";
    path
}

pub fn extract_headers(map: &reqwest::header::HeaderMap) -> Vec<Vec<String>> {
    let mut headers: Vec<Vec<String>> = Vec::new();

    for (key, value) in map.iter() {
        let mut header: Vec<String> = Vec::new();

        header.push(key.to_string());
        header.push(value.to_str().unwrap().to_string());

        headers.push(header);
    }

    return headers;
}

pub fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn prepare_request(req: SendHttpRequest) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();

    let builder = match req.method {
        HttpMethod::GET => client.get(req.url).body(req.body),
        HttpMethod::POST => client.post(req.url).body(req.body),
        HttpMethod::PUT => client.put(req.url).body(req.body),
        HttpMethod::DELETE => client.delete(req.url).body(req.body),
        HttpMethod::HEAD => client.head(req.url).body(req.body),
        HttpMethod::PATCH => client.patch(req.url).body(req.body),
        HttpMethod::OPTIONS => client
            .request(reqwest::Method::OPTIONS, req.url)
            .body(req.body),
        HttpMethod::CONNECT => client
            .request(reqwest::Method::CONNECT, req.url)
            .body(req.body),
    };

    return builder;
}
