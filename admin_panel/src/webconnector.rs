use crate::data;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Post {
    markdown: String,
    title: String,
    tags: String,
}

#[tokio::main]
pub async fn upload_post(
    markdown: String,
    meta_data: Option<data::MetaData>,
) -> Result<(), Box<dyn std::error::Error>> {
    match meta_data {
        Some(meta) => {
            let concatenated_tags = meta.tags.join(" ");
            let data = Post {
                markdown: markdown,
                title: meta.title,
                tags: concatenated_tags,
            };

            let json_data = serde_json::to_string(&data)?;

            send_json("http://localhost:8080/upload_post", &json_data).await?;

            Ok(())
        }
        None => Err("Missing meta data!")?,
    }
}

async fn send_json(url: &str, json_data: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_string())
        .send()
        .await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);

    Ok(())
}
