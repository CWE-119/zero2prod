use actix_web::{web::{get,post}, App, HttpRequest, HttpResponse, HttpServer, Responder
    };

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
        .route("/", get().to(greet))
        .route("/{name}", get().to(greet))
        .route("/health_check", get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
