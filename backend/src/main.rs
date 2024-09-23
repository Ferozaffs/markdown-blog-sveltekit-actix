pub mod database;
pub mod media;
pub mod posts;
pub mod projects;
pub mod uploads;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fs::create_dir_all("assets/images").unwrap();

    let database = database::Database::new().await;
    match database.check_api_keys().await {
        Ok(b) => {
            if !b {
                match database.create_api_key().await {
                    Ok(key) => {
                        println!("No api key present, generating new key!");
                        println!("Api key: {}", key);
                    }
                    Err(e) => println!("Database error: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Database error: {}", e);
        }
    };

    let app_data = web::Data::new(database);

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .service(projects::project_overview)
            .service(projects::project_content)
            .service(projects::project_summary)
            .service(media::get_image)
            .service(posts::posts)
            .service(posts::post_summary)
            .service(posts::post_content)
            .service(uploads::upload_post)
            .service(uploads::upload_image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
