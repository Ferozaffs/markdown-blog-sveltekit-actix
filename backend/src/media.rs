use crate::database::Database;
use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use shared::MetaData;

#[get("/images/{id}")]
async fn get_image(id: web::Path<String>) -> impl Responder {
    let path = format!("assets/images/{}", id);
    NamedFile::open(path)
}

#[get("/markdown/{id}")]
async fn get_markdown(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let id: String = req.match_info().query("id").parse().unwrap();

    let mut markdown: String;
    let mut meta_data = MetaData::default();
    match db.get_post_summary(&id).await {
        Ok(v) => {
            meta_data.id = v.id;
            meta_data.title = v.title;
            meta_data.description = v.description;
            meta_data.post_type = 0;
            meta_data.project = v.project_id;
            meta_data.tags = v.tags.split(',').map(|s| s.to_string()).collect();

            markdown = db.get_post_content(&id).await.unwrap();
        }
        Err(_) => match db.get_project_summary(&id).await {
            Ok(v) => {
                meta_data.id = v.id;
                meta_data.title = v.name;
                meta_data.description = String::from("");
                meta_data.post_type = 1;
                meta_data.project = uuid::Uuid::nil();
                meta_data.tags = vec![];

                markdown = db.get_project_content(&id).await.unwrap()
            }
            Err(_) => return Ok(HttpResponse::BadRequest().body("Bad request")),
        },
    }

    Ok(HttpResponse::Ok().body(markdown))
}
