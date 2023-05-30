use actix_web::{body, http, web, App, HttpRequest, HttpResponse, HttpServer};

pub async fn e404(_req: HttpRequest) -> HttpResponse {
    let body = body::BoxBody::new("Not Found");
    let response: HttpResponse = HttpResponse::new(http::StatusCode::NOT_FOUND).set_body(body);

    return response;
}

#[actix_web::get("/ping")]
pub async fn ping(_req: HttpRequest) -> HttpResponse {
    let body = body::BoxBody::new("pong");
    let response: HttpResponse = HttpResponse::new(http::StatusCode::OK).set_body(body);

    return response;
}

#[actix_web::main]
pub async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .service(ping)
            .default_service(web::post().to(e404))
    });

    let address = "127.0.0.1";
    let port = 8181;

    println!("Starting asset server on http://{}:{}", address, port);
    server.bind((address, port)).unwrap().run().await.unwrap();
}
