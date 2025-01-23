use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::subicron::search_for_subicron::{search_for_subicron, SubicronSearchRes};


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<Vec<SubicronSearchRes>>
}

#[derive(Deserialize, ToSchema, IntoParams)]
struct QueryParams {
    pub search: Option<String>, // Optional query parameter
}


#[utoipa::path(
    get,
    path = "/subicron",
    params(
        ("search" = String, Query, description = "subicon search")
    ),
    responses(
        (status = 200, description = "Signup successful", body = GetSubicronResDocs, example = json!({
            "status": "success",
            "data": [
                {
                    "image_id": "14",
                    "name": "leg photos",
                    "created_at": "DATE",
                    "subicron_id": 2
                },
                {
                    "image_id": "11",
                    "name": "armpit photos",
                    "created_at": "DATE",
                    "subicron_id": 4
                },
                {
                    "image_id": "142",
                    "name": "nudes",
                    "created_at": "DATE",
                    "subicron_id": 3
                }
            ]
        })),
        (status = 401, description = "Unauthorized", body = GetSubicronResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = GetSubicronResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron"
)]
#[get("")]
async fn get_subicron(
    token_data: HttpRequest, 
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let search_query = query.search.clone().unwrap_or("".to_string());

        let subicrons = search_for_subicron(&search_query).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: Some(subicrons),
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: None,
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct GetSubicronResDocs {
    status: &'static str,
    data: Option<Vec<SubicronSearchResDocs>>
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct SubicronSearchResDocs {
    pub image_id: i64,
    pub name: String,
    #[schema(example = "2025-01-22T15:04:05")]
    pub created_at: String,
    pub subicron_id: i64
}