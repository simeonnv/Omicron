use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use urlencoding::encode;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::config;

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponse {
    status: String,
    data: Vec<Subicron>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Subicron {
    pub created_at: String,
    pub image_id: Option<i64>,
    pub name: String,
    pub subicron_id: i64
}  

pub async fn get_subicrons_req(search: String) -> Result<Vec<Subicron>, String> {

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
        let login_response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        
        console::log_1(&serde_json::to_string(&login_response).unwrap().into());

        Ok(login_response.data)
    } else {
        let login_response: LoginResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(login_response.status)
    }
}
