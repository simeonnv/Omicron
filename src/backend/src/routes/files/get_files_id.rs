use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::files::get_file::get_file;
use crate::structs::files::Files;


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<Files>
}


#[utoipa::path(
    get,
    path = "/files/{file_id}",
    params(
        ("file_id" = String, Path, description = "Unique file ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = GetFilesIdResDocs, example = json!({
            "status": "success",
            "data":{
                "file_id": 12,
                "file_blob": "BLOB",
                "size": 66666,
                "file_type": "image/png",
                "account_id": 12,
                "created_at": "TIME NOW",
            }
        })),
        (status = 401, description = "Unauthorized", body = GetFilesIdResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = GetFilesIdResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Files"
)]
#[get("/{file_id}")]
async fn get_files_id(
    token_data: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let file_id = parse_i64(path.to_string(), "invalid file")?;

        let file = get_file(file_id).await?;
        
        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: Some(file),
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: None,
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct GetFilesIdResDocs {
    status: &'static str,
    data: Option<FilesResDocs>,  // Use FilesResDocs instead of self-reference
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct FilesResDocs {
    pub file_id: i64,
    pub file_blob: Vec<u8>,
    pub size: i64,
    pub file_type: String,
    pub account_id: i64,
    pub created_at: String,
}