use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::insure_subicron_exists::insure_subicron_exists;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::posts::search_for_posts::{search_for_posts, PostsSearchRes};


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<Vec<PostsSearchRes>>
}

#[derive(Deserialize, ToSchema, IntoParams)]
struct QueryParams {
    pub search: Option<String>, // Optional query parameter
}


#[utoipa::path(
    get,
    path = "/subicron/{subicron_id}/posts",
    params(
        ("search" = String, Query, description = "subicon search"),
        ("subicron_id" = String, Path, description = "Unique subicron ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = GetSubicronIdPostsResDocs, example = json!({
            "status": "success",
            "data": [
                {
                    "post_id": "64",
                    "header": "war in ukraine is getting worse!",
                    "body": "records say a estimated of 1millions casulties",
                    "embed_id": "2",
                    "poster_id": "1",
                    "subicron_id": "4",
                    "upvotes": "16233",
                    "created_at": "date now idk"
                },
                {
                    "post_id": "63",
                    "header": "idk im unoriginal",
                    "body": "repeat this like 10 times",
                    "embed_id": "3",
                    "poster_id": "6",
                    "subicron_id": "123",
                    "upvotes": "1000",
                    "created_at": "date now idk"
                }
            ]
        })),
        (status = 401, description = "Unauthorized", body = GetSubicronIdPostsResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = GetSubicronIdPostsResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron Posts"
)]
#[get("/{subicron_id}/posts")]
async fn get_subicron_id_posts(
    token_data: HttpRequest,
    path: web::Path<String>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let search_query = query.search.clone().unwrap_or("".to_string());

        let subicron_id = parse_i64(path.to_string(), "invalid subicron")?;

        insure_subicron_exists(subicron_id).await?;

        let subicrons = search_for_posts(&search_query, subicron_id).await?;

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
struct GetSubicronIdPostsResDocs {
    status: &'static str,
    data: Option<Vec<PostsSearchResDocs>>
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct PostsSearchResDocs {
    pub post_id: i64,
    pub header: String,
    pub body: String,
    pub embed_id: Option<i64>,
    pub poster_id: i64,
    pub subicron_id: i64,
    pub upvotes: i64,
    #[schema(example = "2025-01-22T15:04:05")]
    pub created_at: String
}