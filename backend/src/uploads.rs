use crate::database::Database;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use base64::prelude::*;
use lazy_static::lazy_static;
use nanoid::nanoid;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

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

lazy_static! {
    static ref FINGERPRINTS: Mutex<Vec<ImageUploadData>> = {
        let fingerprints = Vec::new();
        Mutex::new(fingerprints)
    };
}

#[post("/upload_post")]
pub async fn upload_post(
    req: HttpRequest,
    db: web::Data<Database>,
    data: web::Json<String>,
) -> impl Responder {
    let mut status = PostResponse {
        status: "Success".to_string(),
        imagerequest: [].to_vec(),
    };

    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            let result = db.is_auth_valid(auth_str).await;
            if let Ok(true) = result {
                let mut text = data.into_inner();

                let mut meta_data = filter_meta(&mut text);
                if meta_data.id == uuid::Uuid::nil() {
                    meta_data.id = uuid::Uuid::new_v4();
                } else {
                    db.delete_post(&meta_data).await;
                }

                let paths = extract_image_paths(text.as_str());
                let unique_paths: Vec<String> = paths
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();

                for p in unique_paths {
                    let id = nanoid!();

                    replace_path(&mut text, &p, &id);

                    let image_data = ImageUploadData {
                        image_url: p,
                        image_fingerprint: id,
                    };

                    let mut fp = FINGERPRINTS.lock().unwrap();
                    fp.push(image_data.clone());

                    status.imagerequest.push(image_data);
                }

                let mut post_image: String = "undefined".to_string();
                if !status.imagerequest.is_empty() {
                    post_image = status
                        .imagerequest
                        .first()
                        .unwrap()
                        .image_fingerprint
                        .clone();
                }

                tokio::spawn(async move {
                    db.save_post(
                        &meta_data,
                        text.as_str(),
                        format!("{}.jpg", post_image).as_str(),
                    )
                    .await
                });

                return HttpResponse::Ok().json(status);
            }
        }
    }

    HttpResponse::Forbidden().finish()
}

#[post("/upload_image")]
pub async fn upload_image(
    req: HttpRequest,
    db: web::Data<Database>,
    data: web::Json<ImageData>,
) -> impl Responder {
    let mut status = "Success".to_string();

    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            let result = db.is_auth_valid(auth_str).await;
            if let Ok(true) = result {
                let mut fp = FINGERPRINTS.lock().unwrap();

                if let Some(index) = fp
                    .iter()
                    .position(|d| d.image_fingerprint == data.fingerprint)
                {
                    fp.remove(index);

                    let fingerprint = data.fingerprint.clone();
                    tokio::spawn(async move {
                        save_image(data.into_inner(), &fingerprint);
                    });
                } else {
                    status = "Failed".to_string()
                }

                return HttpResponse::Ok().json(status);
            }
        }
    }

    HttpResponse::Forbidden().finish()
}

fn filter_meta(text: &mut String) -> shared::MetaData {
    let data = shared::load_meta_data(text);

    let lines = text.lines().skip(data.1);
    let remaining_text = lines.collect::<Vec<&str>>().join("\n");

    text.clear();
    text.push_str(remaining_text.as_str());
    data.0
}

fn extract_image_paths(input: &str) -> Vec<String> {
    let re =
        Regex::new(r"!\[@IMAGE\]\(((?:[a-zA-Z]:)?(?:\\\\|/)?(?:[\w.-]+(?:\\\\|/))*[\w.-]+\.\w+)\)")
            .unwrap();

    re.captures_iter(input)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .collect()
}

fn replace_path(text: &mut String, path: &String, id: &String) {
    let escaped_path = path.replace("\\", "\\\\");
    let expression = format!(r"!\[@IMAGE\]\({}\)", escaped_path);

    let re = Regex::new(&expression).unwrap();

    let result = re
        .replace_all(text, {
            let new_url = format!("http://127.0.0.1:8080/images/{}.jpg", id);
            format!(
                "![{}]({})",
                Path::new(path).file_name().unwrap().to_string_lossy(),
                new_url
            )
        })
        .to_string();

    text.clear();
    text.push_str(result.as_str());
}

fn save_image(data: ImageData, filename: &str) {
    let image_data = BASE64_STANDARD
        .decode(&data.image)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e));

    let file = File::create(format!("./assets/images/{}.jpg", filename))
        .map_err(|e| actix_web::error::ErrorInternalServerError(e));

    if file.is_ok() && image_data.is_ok() {
        let _ = file
            .unwrap()
            .write_all(&image_data.unwrap())
            .map_err(|e| actix_web::error::ErrorInternalServerError(e));
    }
}
