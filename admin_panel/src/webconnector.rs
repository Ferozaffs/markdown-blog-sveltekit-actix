use crate::data;

use reqwest::{header, Client};

const API_KEY: &str = "offfdkTJrYwDSRqJzAsuKGgbYzbP6Xe2";

#[tokio::main]
pub async fn upload_post(
    markdown: String,
    meta_data: Option<data::MetaData>,
    server: &str,
    api_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match meta_data {
        Some(_meta) => {
            let data = markdown;

            let json_data = serde_json::to_string(&data)?;

            let url = format!("http://{}/upload_post", server);
            send_json(url.as_str(), api_token, &json_data).await?;

            Ok(())
        }
        None => Err("Missing meta data!")?,
    }
}

async fn send_json(url: &str, api_token: &str, json_data: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res = client
        .post(url)
        .header(header::AUTHORIZATION, format!("{}", api_token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(json_data.to_string())
        .send()
        .await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);

    Ok(())
}
