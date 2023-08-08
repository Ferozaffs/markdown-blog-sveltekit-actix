pub mod database;
pub mod projects;
pub mod media;

use std::fs;
use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello world!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fs::create_dir_all("images").unwrap();

    let database = database::Database::new().await;
    let app_data = web::Data::new(database);

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .send_wildcard();

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .service(hello)
            .service(projects::project_overview)
            .service(projects::project_content)
            .service(media::get_image)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}