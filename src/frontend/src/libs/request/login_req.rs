use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config;



#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    status: String,
    data: String
}


pub async fn login_req(username: String, password: String) -> Result<String, String> {
    let client = Client::new();
    let request = LoginRequest { username, password };

    let response = client
        .post(&format!("{}/auth/login", config::BACKEND_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(login_response.data)
    } else {
        let login_response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(login_response.status)
    }
}