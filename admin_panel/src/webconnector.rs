use crate::data;

use reqwest::{header, Client};

const API_KEY: &str = "offfdkTJrYwDSRqJzAsuKGgbYzbP6Xe2";

#[tokio::main]
pub async fn upload_post(
    markdown: String,
    meta_data: Option<data::MetaData>,
) -> Result<(), Box<dyn std::error::Error>> {
    match meta_data {
        Some(_meta) => {
            let data = markdown;

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
        .header(header::AUTHORIZATION, format!("{}", API_KEY))
        .header(header::CONTENT_TYPE, "application/json")
        .body(json_data.to_string())
        .send()
        .await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);

    Ok(())
}
