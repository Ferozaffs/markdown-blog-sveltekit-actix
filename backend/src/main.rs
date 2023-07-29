pub mod projects;
use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
        .allow_any_origin()
        .send_wildcard();

        App::new()
            .wrap(cors)
            .service(hello)
            .service(projects::project_overview)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}