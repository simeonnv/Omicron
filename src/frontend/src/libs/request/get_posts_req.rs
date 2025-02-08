use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::{config, libs::structs::post::PostStruct};

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponse {
    status: String,
    data: Vec<PostStruct>
}

pub async fn get_posts_req(search: String, subicron_id: i64, page: u64) -> Result<Vec<PostStruct>, String> {

    console::log_1(&"fetching Posts!".into());

    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

    let mut url = Url::parse(&format!("{}/subicron/{}/posts", config::BACKEND_URL, subicron_id))
        .map_err(|e| e.to_string())?;
    url.query_pairs_mut().append_pair("search", &search);
    url.query_pairs_mut().append_pair("page", &page.to_string());

    let response = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        
        console::log_1(&serde_json::to_string(&response).unwrap_or("".to_string()).into());

        Ok(response.data)
    } else {
        let response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(response.status)
    }
}
