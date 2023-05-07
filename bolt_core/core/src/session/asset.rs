use actix_web::{body, http, web, App, HttpRequest, HttpResponse, HttpServer};

use crate::utils::open_browser;
use crate::utils::get_dist;

pub async fn e404(_req: HttpRequest) -> HttpResponse {
    let body = body::BoxBody::new("Not Found");
    let response: HttpResponse = HttpResponse::new(http::StatusCode::NOT_FOUND).set_body(body);

    return response;
}


#[actix_web::main]
pub async fn launch_asset_server(port: u16, address: String) {
    std::thread::spawn(move || {
        println!("opening browser");
        open_browser("http://localhost:".to_string() + &port.to_string());
    });

    let asset_server = HttpServer::new(|| {
        let dist_path = get_dist();

        App::new()
            .service(actix_files::Files::new("/", dist_path).index_file("index.html"))
            .default_service(web::post().to(e404))
    });

    println!("Starting asset server on {} port {}", address, port);
    asset_server
        .bind((address, port))
        .unwrap()
        .run()
        .await
        .unwrap();
}
