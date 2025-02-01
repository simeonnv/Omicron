use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config;



#[derive(Serialize)]
struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct SignupResponse {
    status: String,
    data: String
}


pub async fn signup_req(username: String, password: String) -> Result<String, String> {
    let client = Client::new();
    let request = SignupRequest { username, password };

    let response = client
        .post(&format!("{}/auth/signup", config::BACKEND_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let signup_response: SignupResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(signup_response.data)
    } else {
        let signup_response: SignupResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(signup_response.status)
    }
}