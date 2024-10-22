use crate::data::{self, ServerContentSummary};

use base64::prelude::*;
use reqwest::{blocking::get, header, Client, Response};
use serde::{Deserialize, Serialize};
use std::fs;

const API_KEY: &str = "offfdkTJrYwDSRqJzAsuKGgbYzbP6Xe2";

#[derive(Deserialize)]
struct ImageUploadData {
    image_url: String,
    image_fingerprint: String,
}

#[derive(Deserialize)]
struct PostResponse {
    status: String,
    imagerequest: Vec<ImageUploadData>,
}

#[derive(Serialize)]
struct ImageData {
    fingerprint: String,
    image: String,
}

pub async fn upload_post(
    markdown: String,
    meta_data: Option<shared::MetaData>,
    server: &str,
    api_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match meta_data {
        Some(_meta) => {
            let data = markdown;

            let json_data = serde_json::to_string(&data)?;

            let url = format!("http://{}/upload_post", server);
            let res = send_json(url.as_str(), api_token, &json_data).await;
            match res {
                Ok(r) => {
                    println!("Status: {}", r.status());
                    if r.status().is_success() {
                        let body = r.text().await?;
                        let json_res: PostResponse = serde_json::from_str(&body).unwrap();

                        let url = format!("http://{}/upload_image", server);
                        for i in json_res.imagerequest.iter() {
                            send_json(url.as_str(), api_token, get_image_data(i).as_str()).await?;
                        }
                    }
                }
                Err(e) => println!("Upload error: {}", e),
            }

            Ok(())
        }
        None => Err("Missing meta data!")?,
    }
}

async fn send_json(
    url: &str,
    api_token: &str,
    json_data: &str,
) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    client
        .post(url)
        .header(header::AUTHORIZATION, format!("{}", api_token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(json_data.to_string())
        .send()
        .await
}

fn get_image_data(img: &ImageUploadData) -> String {
    let image_bytes = fs::read(img.image_url.as_str()).unwrap();

    let encoded_image = BASE64_STANDARD.encode(&image_bytes);

    let image_data = ImageData {
        fingerprint: img.image_fingerprint.clone(),
        image: encoded_image,
    };

    serde_json::to_string(&image_data).unwrap()
}

pub fn load_content_summary(server: &str) -> data::ServerContentSummary {
    let mut server_content_summary = ServerContentSummary::default();

    match get(format!("http://{}/projectoverview", server)) {
        Ok(res) => match res.json::<shared::ProjectOverview>() {
            Ok(v) => server_content_summary.projects = v,
            Err(_) => (),
        },
        Err(_) => (),
    }

    match get(format!("http://{}/posts/*", server)) {
        Ok(res) => match res.json::<Vec<shared::PostSummary>>() {
            Ok(v) => server_content_summary.posts = v,
            Err(_) => (),
        },
        Err(_) => (),
    }

    return server_content_summary;
}
