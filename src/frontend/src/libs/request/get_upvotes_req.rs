use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::{config, libs::structs::upvotes_struct::UpvotesStruct};

#[derive(Deserialize, Serialize, Debug)]
struct GetUpvotesResponse {
    status: String,
    data: Option<UpvotesStruct>
}



pub async fn get_upvotes_req(subicron_id: i64, post_id: i64) -> Result<UpvotesStruct, String> {

    console::log_1(&"fetching upvotes!".into());

    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

    let url = Url::parse(&format!("{}/subicron/{}/{}/upvote", config::BACKEND_URL, subicron_id, post_id))
        .map_err(|e| e.to_string())?;

    let response = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let response: GetUpvotesResponse = response.json().await.map_err(|e| e.to_string())?;
        
        console::log_1(&serde_json::to_string(&response).unwrap().into());

        match response.data {
            Some(e) => return Ok(e),
            None => return Err(response.status)
        }

    } else {
        let response: GetUpvotesResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(response.status)
    }
}
