//Autism.rs

use tokio;
use serde_json;
use reqwest;
use anyhow;

#[tokio::main]
async fn main() {
    let client_id: &str = "xxx";
    let client_secret: &str = "xxx";
    let callback_uri: &str = "https://eve-api-callback";
    let authorization_code: &str = "xxx";

    match get_token(client_id, client_secret, callback_uri, authorization_code).await {
        Ok(access_token) => {
            println!("Access Token: {}", access_token);
            println!("Done");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

async fn get_token(
    client_id: &str,
    client_secret: &str,
    callback_uri: &str,
    authorization_code: &str,
) -> Result<String, anyhow::Error> {
    let token_url: &str = "https://login.eveonline.com/oauth/token";

    let client: reqwest::Client = reqwest::Client::new();
    let response = client
        .post(token_url)
        .basic_auth(client_id, Some(client_secret))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", authorization_code),
            ("redirect_uri", callback_uri),
        ])
        .send()
        .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            let token_info: serde_json::Value = serde_json::from_str(&response_text)?;
            let access_token = token_info["access_token"].as_str().unwrap_or_default().to_string();
        
            Ok(access_token)
        } else {
            // Use a different name for the error variable to avoid conflicts
            Err(anyhow::anyhow!("Request failed with Status: {}", response.status()))
        }
    }