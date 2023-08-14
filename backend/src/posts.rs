use actix_web::{get, web, Responder, Result, HttpRequest};
use serde::Serialize;
use crate::database::Database;

#[derive(Serialize, Debug)]
struct PostSummary {
    title: String,
    image: String,
}

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
        let post = PostSummary {
            title: row.get(0),
            image: row.get(1),
        };

        posts.push(post);
    }

    Ok(web::Json(posts))
}