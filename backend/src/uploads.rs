use crate::database::Database;
use actix_web::{post, web, HttpResponse, Responder};
use base64::prelude::*;
use lazy_static::lazy_static;
use nanoid::nanoid;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
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
    static ref FINGERPRINTS: Mutex<Vec<ImageUploadData>> = {
        let fingerprints = Vec::new();
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

    remove_meta(&mut data.markdown);
    for p in unique_paths {
        let id = nanoid!();

        replace_path(&mut data.markdown, &p, &id);

        let image_data = ImageUploadData {
            image_url: p,
            image_fingerprint: id,
        };

        let mut fp = FINGERPRINTS.lock().unwrap();
        fp.push(image_data.clone());

        status.imagerequest.push(image_data);
    }

    let mut id = "undefined".to_string();
    if !status.imagerequest.is_empty() {
        id = status
            .imagerequest
            .first()
            .unwrap()
            .image_fingerprint
            .clone();
    }

    tokio::spawn(async move {
        save_post(db, data.into_inner(), id).await;
    });

    HttpResponse::Ok().json(status)
}

#[post("/upload_image")]
pub async fn upload_image(_db: web::Data<Database>, data: web::Json<ImageData>) -> impl Responder {
    let mut status = ImageResponse {
        status: "Success".to_string(),
    };

    let mut fp = FINGERPRINTS.lock().unwrap();

    if let Some(index) = fp
        .iter()
        .position(|d| d.image_fingerprint == data.fingerprint)
    {
        let filename = extract_filename(fp[index].image_url.as_str());
        fp.remove(index);

        tokio::spawn(async move {
            save_image(data.into_inner(), &filename).await;
        });
    } else {
        status.status = "Failed".to_string()
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

fn remove_meta(text: &mut String) {
    let lines = text.lines().skip(4);
    let remaining_text = lines.collect::<Vec<&str>>().join("\n");

    text.clear();
    text.push_str(remaining_text.as_str());
}

async fn save_post(db: web::Data<Database>, data: PostData, image_fingerprint: String) {
    let result = db
        .save_post(
            data.title.as_str(),
            data.tags.as_str(),
            data.markdown.as_str(),
            image_fingerprint.as_str(),
        )
        .await;

    match result {
        Ok(value) => println!("Uploaded post: {}", value),
        Err(e) => println!("Error uploading post: {}", e),
    }
}

async fn save_image(data: ImageData, filename: &str) {
    let image_data = BASE64_STANDARD
        .decode(&data.image)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e));

    let file = File::create(format!("./assets/{}", filename))
        .map_err(|e| actix_web::error::ErrorInternalServerError(e));

    if file.is_ok() && image_data.is_ok() {
        file.unwrap()
            .write_all(&image_data.unwrap())
            .map_err(|e| actix_web::error::ErrorInternalServerError(e));
    }
}
