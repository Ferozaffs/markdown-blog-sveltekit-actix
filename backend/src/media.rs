use actix_files::NamedFile;
use actix_web::{get, web, Responder};

#[get("/images/{id}")]
async fn get_image(id: web::Path<String>) -> impl Responder {
    let path = format!("images/{}", id);
    NamedFile::open(path)
}