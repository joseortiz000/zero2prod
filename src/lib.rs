use std::net::TcpListener;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

#[derive(serde::Serialize, serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>) -> web::Json<FormData>{
    web::Json(form.into_inner())
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}