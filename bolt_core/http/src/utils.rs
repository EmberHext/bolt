use crate::Method;
use crate::Request;

use std::time::SystemTime;

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

pub fn prepare_request(req: Request) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();

    let builder = match req.method {
        Method::GET => client.get(req.url).body(req.body),
        Method::POST => client.post(req.url).body(req.body),
        Method::PUT => client.put(req.url).body(req.body),
        Method::DELETE => client.delete(req.url).body(req.body),
        Method::HEAD => client.head(req.url).body(req.body),
        Method::PATCH => client.patch(req.url).body(req.body),
        Method::OPTIONS => client
            .request(reqwest::Method::OPTIONS, req.url)
            .body(req.body),
        Method::CONNECT => client
            .request(reqwest::Method::CONNECT, req.url)
            .body(req.body),
    };

    return builder;
}

pub fn open_browser(link: String) {
    std::thread::sleep(std::time::Duration::from_secs(2));

    webbrowser::open(&link).unwrap();
}

pub fn get_dist() -> String {
    get_home() + "dist/"
}

pub fn get_home() -> String {
    let path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/bolt/";
    path
}


pub fn __bolt_log(log: String) {
    println!("LOG: {log}");
}