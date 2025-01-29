use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::posts::get_post_from_id::{get_post_from_id, PostSearchRes};


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<PostSearchRes>
}

#[utoipa::path(
    get,
    path = "/subicron/{subicron_id}/posts/{post_id}",
    params(
        ("subicron_id" = String, Path, description = "Unique subicron ID"),
        ("post_id" = String, Path, description = "Unique post ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = GetSubicronIdPostsResDocs, example = json!({
            "status": "success",
            "data":
                {
                    "post_id": "64",
                    "header": "war in ukraine is getting worse!",
                    "body": "records say a estimated of 1millions casulties",
                    "embed_id": "2",
                    "poster_id": "1",
                    "subicron_id": "4",
                    "upvotes": "16233",
                    "created_at": "date now idk"
                }
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
#[get("/{subicron_id}/posts/{post_id}")]
async fn get_subicron_id_posts_id(
    token_data: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let subicron_id = parse_i64(path.0.clone(), "invalid subicron")?;
        let post_id = parse_i64(path.1.clone(), "invalid post_id")?;

        let post = get_post_from_id(post_id, subicron_id).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: post,
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