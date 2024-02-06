// API Call
async fn get_token(
    client_id: &str,
    client_secret: &str,
    callback_uri: &str,
    authorization_code: &str,
) -> Result<String, anyhow::Error> {

    // Client networking
    let client = reqwest::Client::new();

    // Handle response
    let response = client
        .post(TOKEN_URL)
        .basic_auth(client_id, Some(client_secret))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", authorization_code),
            ("redirect_uri", callback_uri),
        ])
        .send()
        .await?
        .error_for_status()
        .context("Request Failed")?;

    // Response [Response] -> Response [string?]
    let response_text = response.text().await?;

    // Response_text[String] -> token_info[JSON_SHIT]
    let token_info: serde_json::Value = serde_json::from_str(&response_text)
        .context("Failed to parse JSON")?;

    // Access Token[String] -> Same Shit[String]
    let access_token = token_info["access_token"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    Ok(access_token)
}    