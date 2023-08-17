pub mod database;
pub mod projects;
pub mod media;
pub mod posts;

use std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fs::create_dir_all("assets/images").unwrap();

    let database = database::Database::new().await;
    let app_data = web::Data::new(database);

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .send_wildcard();

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .service(projects::project_overview)
            .service(projects::project_content)
            .service(projects::project_summary)
            .service(media::get_image)
            .service(posts::posts)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}