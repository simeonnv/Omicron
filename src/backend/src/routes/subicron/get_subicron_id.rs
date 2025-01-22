use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::subicron::get_subicron_from_id::get_subicron_from_id;
use crate::libs::subicron::search_for_subicron::SubicronSearchRes;


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<SubicronSearchRes>
}


#[utoipa::path(
    get,
    path = "/subicron/{subicron_id}",
    params(
        ("subicron_id" = String, Path, description = "Unique subicron ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = GetSubicronIdResDocs, example = json!({
            "status": "success",
            "data":{
                "image_id": "14",
                "name": "leg photos",
                "created_at": "DATE",
                "subicron_id": 2
            }
        })),
        (status = 401, description = "Unauthorized", body = GetSubicronIdResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = GetSubicronIdResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron"
)]
#[get("/{subicron_id}")]
async fn get_subicron_id(
    token_data: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let subicron = get_subicron_from_id(&path).await?;

        if subicron.is_none() {
            return Ok(HttpResponse::NotFound().json(Res {
                status: "Subicron doenst exist",
                data: None,
            }))
        }
        
        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: subicron,
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: None,
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct GetSubicronIdResDocs {
    status: &'static str,
    data: Option<SubicronGetResDocs>
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct SubicronGetResDocs {
    pub image_id: i64,
    pub name: String,
    #[schema(example = "2025-01-22T15:04:05")]
    pub created_at: String,
    pub subicron_id: i64
}