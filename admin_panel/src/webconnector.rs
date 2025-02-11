use crate::data::{self, ServerContentSummary};

use base64::prelude::*;
use reqwest::{blocking, blocking::get, blocking::Client, header};
use serde::{Deserialize, Serialize};
use std::fs;

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

#[derive(Serialize)]
struct ProjectCategory {
    title: String,
    description: String,
}

pub fn upload_post(
    markdown: String,
    server: &str,
    api_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = markdown;

    let json_data = serde_json::to_string(&data)?;

    let url = format!("http://{}/upload_post", server);
    let res = send_json(url.as_str(), api_token, &json_data);
    match res {
        Ok(r) => {
            println!("Status: {}", r.status());
            if r.status().is_success() {
                let body = r.text().unwrap();
                let json_res: PostResponse = serde_json::from_str(&body).unwrap();

                let url = format!("http://{}/upload_image", server);
                for i in json_res.imagerequest.iter() {
                    let _ = send_json(url.as_str(), api_token, get_image_data(i).as_str());
                }
            }
        }
        Err(e) => println!("Upload error: {}", e),
    }

    Ok(())
}

fn send_json(
    url: &str,
    api_token: &str,
    json_data: &str,
) -> Result<blocking::Response, reqwest::Error> {
    let client = Client::new();
    client
        .post(url)
        .header(header::AUTHORIZATION, format!("{}", api_token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(json_data.to_string())
        .send()
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

    let mut project_overview: Option<shared::ProjectOverview> = None;
    match get(format!("http://{}/projectoverview", server)) {
        Ok(res) => match res.json::<shared::ProjectOverview>() {
            Ok(v) => {
                println!("Projects found");
                project_overview = Some(v)
            }

            Err(_) => (),
        },
        Err(e) => println!("ERROR: {}", e.to_string()),
    }

    match project_overview {
        Some(po) => {
            for category in po.categories.iter() {
                server_content_summary.categories.push(category.clone());
                for project in category.children.iter() {
                    server_content_summary.projects.push(project.clone());
                }
            }
        }
        None => println!("No projects found"),
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

pub fn get_markdown(server: &str, id: uuid::Uuid) -> String {
    match get(format!("http://{}/markdown/{}", server, id.to_string())) {
        Ok(res) => res.text().unwrap(),
        Err(_) => String::from(""),
    }
}

pub fn upload_project_category(
    title: &str,
    description: &str,
    server: &str,
    api_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = ProjectCategory {
        title: title.to_string(),
        description: description.to_string(),
    };

    let json_data = serde_json::to_string(&data)?;

    let url = format!("http://{}/upload_project_category", server);
    let res = send_json(url.as_str(), api_token, &json_data);
    match res {
        Ok(r) => {
            println!("Status: {}", r.status());
        }
        Err(e) => println!("Upload error: {}", e),
    }

    Ok(())
}
