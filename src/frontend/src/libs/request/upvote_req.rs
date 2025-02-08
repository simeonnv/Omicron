use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::config;

#[derive(Deserialize, Serialize, Debug)]
struct PostUpvoteResponse {
    status: String,
    data: String
}

pub async fn upvote_req(subicron_id: i64, post_id: i64) -> Result<String, String> {

    console::log_1(&"fetching Posts!".into());

    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

    let url = Url::parse(&format!("{}/subicron/{}/posts/{}/upvote", config::BACKEND_URL, subicron_id, post_id))
        .map_err(|e| e.to_string())?;

    let response = client
        .post(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let response: PostUpvoteResponse = response.json().await.map_err(|e| e.to_string())?;
        
        console::log_1(&serde_json::to_string(&response).unwrap_or("".to_string()).into());

        Ok(response.data)
    } else {
        let response: PostUpvoteResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(response.status)
    }
}
