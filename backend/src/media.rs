use crate::database::Database;
use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};

#[get("/images/{id}")]
async fn get_image(id: web::Path<String>) -> impl Responder {
    let path = format!("assets/images/{}", id);
    NamedFile::open(path)
}

#[get("/markdown/{id}")]
async fn get_markdown(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let id: String = req.match_info().query("id").parse().unwrap();

    let mut markdown = String::from("No post found");

    let mut found = false;
    match db.get_post_summary(&id).await {
        Ok(v) => {
            if v.id != uuid::Uuid::nil() {
                found = true;

                let meta_data = shared::MetaData {
                    id: v.id,
                    title: v.title,
                    description: v.description,
                    post_type: 0,
                    project: v.project_id,
                    tags: v.tags.split(',').map(|s| s.to_string()).collect(),
                };

                markdown = db.get_post_content(&id).await.unwrap();
                shared::store_meta_data(&mut markdown, meta_data);
            }
        }
        Err(_) => (),
    }

    if found == false {
        match db.get_project_summary(&id).await {
            Ok(v) => {
                let meta_data = shared::MetaData {
                    id: v.id,
                    title: v.title,
                    description: String::from(""),
                    post_type: 1,
                    project: uuid::Uuid::nil(),
                    tags: vec![],
                };

                markdown = db.get_project_content(&id).await.unwrap();
                shared::store_meta_data(&mut markdown, meta_data);
            }
            Err(_) => return Ok(HttpResponse::BadRequest().body("Bad request")),
        }
    }

    Ok(HttpResponse::Ok().body(markdown))
}
