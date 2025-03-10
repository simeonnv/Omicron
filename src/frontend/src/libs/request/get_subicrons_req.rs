use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::{config, libs::structs::subicron::SubicronStruct};

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponse {
    status: String,
    data: Vec<SubicronStruct>
}

pub async fn get_subicrons_req(search: String) -> Result<Vec<SubicronStruct>, String> {

    console::log_1(&"fetching subicrons!".into());

    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

    let mut url = Url::parse(&format!("{}/subicron", config::BACKEND_URL))
        .map_err(|e| e.to_string())?;
    url.query_pairs_mut().append_pair("search", &search);

    let response = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        
        console::log_1(&serde_json::to_string(&response).unwrap().into());

        Ok(response.data)
    } else {
        let response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(response.status)
    }
}
