// App.rs
// Michael Steer: 2024
// 
// Don't delete above whitespace

use tokio;      // Multithreading
use serde_json; // JSON packing/unpacking
use reqwest;    // Networking
use anyhow::{self, Context};     // Error Wrangling

const TOKEN_URL: &str = "https://login.eveonline.com/oauth/xxx";

pub struct App {

}

impl App {

}

 

// Main function
#[tokio::main]
async fn main() {

    // TODO: Move
    let client_id: &str = "xxx";                                    // Eve Client ID
    let client_secret: &str = "xxx";                                // Eve Client Secret (TODO: Pull as SysVar or File)
    let callback_uri: &str = "https://eve-api-callback";            // CCP Callback ID
    let authorization_code: &str = "xxx";                           // oAuth2 ID

    // How did we do? 🤔🤔🤔
    match get_token(client_id, client_secret, callback_uri, authorization_code).await {
        Ok(access_token) => {
            println!("Access Token: {}", access_token);              // TODO: Rigorously check
            println!("Done");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
