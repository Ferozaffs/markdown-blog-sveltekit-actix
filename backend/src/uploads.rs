use crate::database::Database;
use actix_web::{post, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use nanoid::nanoid;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Deserialize)]
struct PostData {
    markdown: String,
    title: String,
    tags: String,
}

#[derive(Deserialize)]
struct ImageData {
    fingerprint: String,
    image: String,
}

#[derive(Clone, Serialize, Debug)]
struct ImageUploadData {
    image_url: String,
    image_fingerprint: String,
}

#[derive(Serialize, Debug)]
struct PostResponse {
    status: String,
    imagerequest: Vec<ImageUploadData>,
}

#[derive(Serialize, Debug)]
struct ImageResponse {
    status: String,
}

lazy_static! {
    static ref FINGERPRINTS: Mutex<HashSet<String>> = {
        let mut fingerprints = HashSet::new();
        Mutex::new(fingerprints)
    };
}

#[post("/upload_post")]
pub async fn upload_post(db: web::Data<Database>, mut data: web::Json<PostData>) -> impl Responder {
    let mut status = PostResponse {
        status: "Success".to_string(),
        imagerequest: [].to_vec(),
    };

    let paths = extract_image_paths(data.markdown.as_str());
    let unique_paths: Vec<String> = paths
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    for p in unique_paths {
        let id = nanoid!();

        replace_path(&mut data.markdown, &p, &id);

        let mut fp = FINGERPRINTS.lock().unwrap();
        fp.insert(id.clone());

        status.imagerequest.push(ImageUploadData {
            image_url: p,
            image_fingerprint: id,
        });
    }

    //tokio::spawn(async move {
    //    save_post(data.into_inner()).await;
    //});

    HttpResponse::Ok().json(status)
}

#[post("/upload_image")]
pub async fn upload_image(db: web::Data<Database>, data: web::Json<ImageData>) -> impl Responder {
    let mut status = ImageResponse {
        status: "Success".to_string(),
    };

    let mut fp = FINGERPRINTS.lock().unwrap();
    if !fp.contains(&data.fingerprint) {
        status.status = "Failed".to_string()
    } else {
        fp.remove(&data.fingerprint);
        //tokio::spawn(async move {
        //    save_image(data.into_inner()).await;
        //});
    }

    HttpResponse::Ok().json(status)
}

fn extract_image_paths(input: &str) -> Vec<String> {
    let re = Regex::new(r"!\[@IMAGE\]\((.*)\)").unwrap();

    re.captures_iter(input)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .collect()
}

fn extract_filename(path: &str) -> String {
    path.rsplit('/').next().unwrap_or("").to_string()
}

fn replace_path(text: &mut String, path: &String, id: &String) {
    let expression = format!(r"!\[@IMAGE\]\({}\)", path);

    let re = Regex::new(&expression).unwrap();

    let result = re
        .replace_all(text, {
            let new_url = format!("http://127.0.0.1/images/{}", id);
            format!("![{}]({})", extract_filename(path), new_url)
        })
        .to_string();

    text.clear();
    text.push_str(result.as_str());
}
