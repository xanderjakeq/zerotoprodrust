use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use serde::{Serialize, Deserialize};

async fn health_check() -> impl Responder {
    return HttpResponse::Ok().finish();
}


#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    return HttpResponse::Ok().finish();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
