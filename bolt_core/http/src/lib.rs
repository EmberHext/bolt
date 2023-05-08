mod utils;

use bolt_common::prelude::*;
use std::time::SystemTime;

pub async fn http_send(mut req: SendHttpRequest) -> SendHttpResponse {
    if !req.url.contains("http") {
        let new_url = "http://".to_string() + &req.url;

        req.url = new_url;
    }

    let mut request = prepare_request(req.clone());

    for h in req.headers {
        if h[0] != "" && h[1] != "" {
            request = request.header(h[0].clone(), h[1].clone());
        }
    }

    let start = get_timestamp();
    let response = request.send().await;
    let end = get_timestamp();

    let mut http_response = match response {
        Ok(resp) => {
            let mut new_response = SendHttpResponse::new();

            new_response.headers = extract_headers(resp.headers());
            new_response.status = resp.status().as_u16();
            new_response.time = (end - start) as u32;
            new_response.body = resp.text().await.unwrap();
            new_response.size = new_response.body.len() as u64;

            for header in &new_response.headers {
                if header[0] == "content-type" {
                    if header[1].contains("application/json") {
                        new_response.response_type = SendHttpResponseType::JSON;
                    }
                }
            }

            new_response
        }

        Err(err) => {
            let mut err_resp = SendHttpResponse::new();

            err_resp.failed = true;

            err_resp.body = err.to_string();

            err_resp
        }
    };

    http_response.request_index = req.request_index;

    return http_response;
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

pub fn launch_http_service() {
    println!("Starting HTTP service");

    std::thread::park();
}
