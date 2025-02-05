use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Serialize)]
struct PostCommentRequest {
    pub file_blob: Vec<u8>,
}

#[derive(Deserialize)]
struct PostCommentResponse {
    status: String,
    data: Option<i64>,
}

pub async fn post_file_req(file_blob: Vec<u8>) -> Result<i64, String> {
    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

        let url = Url::parse(&format!("{}/files", config::BACKEND_URL))
        .map_err(|e| e.to_string())?;

    let request = PostCommentRequest{ file_blob: file_blob };

    let response = client
        .post(url)  // Changed from .get to .post since you're sending a POST request
        .bearer_auth(token)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let login_response: PostCommentResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(login_response.data.unwrap_or_default())
    } else {
        let login_response: PostCommentResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(login_response.status)
    }
}