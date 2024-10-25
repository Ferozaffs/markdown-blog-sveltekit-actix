use crate::database::Database;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use shared;

#[get("/posts/{tags}")]
async fn posts(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let tags: String = req.match_info().query("tags").parse().unwrap();
    let split_tags = tags.split(",");
    let mut vec: Vec<&str> = split_tags.collect();
    if *vec.first().unwrap() == "*" {
        vec = vec![];
    }

    let result = db.get_posts(vec).await;

    let mut posts = Vec::new();
    for row in result.unwrap() {
        let post = shared::PostSummary {
            id: row.get(0),
            title: row.get(1),
            image: row.get(2),
            date: row.get(3),
            description: row.get(4),
            tags: row.get(5),
            project_id: row.get(6),
        };

        posts.push(post);
    }

    Ok(web::Json(posts))
}

#[get("/postcontent/{id}")]
async fn post_content(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let id: String = req.match_info().query("id").parse().unwrap();
    match db.get_post_content(&id).await {
        Ok(content) => Ok(HttpResponse::Ok().body(content)),
        Err(_) => Ok(HttpResponse::BadRequest().body("Bad request")),
    }
}

#[get("/postsummary/{id}")]
async fn post_summary(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let id: String = req.match_info().query("id").parse().unwrap();
    match db.get_post_summary(&id).await {
        Ok(post) => Ok(web::Json(serde_json::to_value(&post).unwrap())),
        Err(_) => {
            let error_response = serde_json::json!({
                "error": "Bad request",
                "message": "Unable to fetch the post summary."
            });
            Ok(web::Json(error_response))
        }
    }
}
