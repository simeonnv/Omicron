use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::{config, libs::structs::file::FileStruct};

#[derive(Deserialize, Serialize, Debug)]
struct GetFileResponse {
    status: String,
    data: Option<FileStruct>
}

pub async fn get_file_req(file_id: i64) -> Result<FileStruct, String> {

    console::log_1(&"fetching file!".into());

    let client = Client::new();
    let token = LocalStorage::get::<String>("token").map_err(|e| e.to_string())?;

    let url = Url::parse(&format!("{}/files/{}", config::BACKEND_URL, file_id))
        .map_err(|e| e.to_string())?;

    let response = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let response: GetFileResponse = response.json().await.map_err(|e| e.to_string())?;
        
        console::log_1(&serde_json::to_string(&response).unwrap_or("".to_string()).into());

        if response.data.is_none() {
            Err(response.status)
        } else {
            Ok(response.data.unwrap_or_default())
        }
        
    } else {
        let response: GetFileResponse = response.json().await.map_err(|e| e.to_string())?;
        Err(response.status)
    }
}
