use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| App::new().service(echo))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
