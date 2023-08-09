use actix_web::{get, web, Responder, Result, HttpRequest};
use serde::Serialize;
use crate::database::Database;

#[derive(Serialize, Debug)]
struct PostSummary {
    name: String,
    image: String,
}

#[get("/posts/{tags}")]
async fn posts(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let tags: String = req.match_info().query("tags").parse().unwrap();
    let split_tags = tags.split(",");

    let result = db.get_posts(split_tags.collect()).await;
   
    let mut posts = Vec::new();
    for row in result.unwrap() {
        let post = PostSummary {
            name: row.get(0),
            image: row.get(1),
        };

        posts.push(post);
    }

    Ok(web::Json(posts))
}