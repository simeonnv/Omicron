use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::{config, libs::structs::comment::CommentStruct};

#[derive(Deserialize, Serialize, Debug)]
struct GetCommentsResponse {
    status: String,
    data: Vec<CommentStruct>
}

pub async fn get_comments_req(subicron_id: i64, post_id: i64) -> Result<Vec<CommentStruct>, String> {

    console::log_1(&"fetching comments!".into());

    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

    let url = Url::parse(&format!("{}/subicron/{}/posts/{}/comments", config::BACKEND_URL, subicron_id, post_id))
        .map_err(|e| e.to_string())?;

    let response = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let response: GetCommentsResponse = response.json().await.map_err(|e| e.to_string())?;
        
        console::log_1(&serde_json::to_string(&response).unwrap().into());

        Ok(response.data)
    } else {
        let response: GetCommentsResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(response.status)
    }
}
