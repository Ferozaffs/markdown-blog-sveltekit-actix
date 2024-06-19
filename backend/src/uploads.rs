use crate::database::Database;
use actix_web::{post, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PostData {
    markdown: String,
    title: String,
    tags: String,
}

#[derive(Clone, Serialize, Debug)]
struct ImageUploadData {
    image_url: String,
    image_fingerprint: String,
}

#[derive(Serialize, Debug)]
struct Status {
    status: String,
    imagerequest: Vec<ImageUploadData>,
}

#[post("/upload_post")]
pub async fn upload_post(db: web::Data<Database>, data: web::Json<PostData>) -> impl Responder {
    let status = Status {
        status: "Success".to_string(),
        imagerequest: [].to_vec(),
    };

    HttpResponse::Ok().json(status)
}
