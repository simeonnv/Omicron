use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Serialize)]
struct PostCommentRequest {
    text: String,
    embed_id: Option<i64>,
}

#[derive(Deserialize)]
struct PostCommentResponse {
    status: String,
    data: Option<String>,
}

pub async fn post_comment_req(subicron_id: i64, post_id: i64, text: String, embed_id_option: Option<i64>) -> Result<String, String> {
    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

    let url = Url::parse(&format!("{}/subicron/{}/posts/{}/comments", config::BACKEND_URL, subicron_id, post_id))
        .map_err(|e| e.to_string())?;

    let request = PostCommentRequest {
        text,
        embed_id: embed_id_option,
    };

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