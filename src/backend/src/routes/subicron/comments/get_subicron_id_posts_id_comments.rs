use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::insure_post_exists::insure_post_exists;
use crate::libs::auth::insure_subicron_exists::insure_subicron_exists;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::comments::get_comments_on_post::{get_comments_on_post, Comment};


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<Vec<Comment>>
}

#[derive(Deserialize, ToSchema, IntoParams)]
struct QueryParams {
    pub page: Option<i64>, // Optional query parameter
}

#[utoipa::path(
    get,
    path = "/subicron/{subicron_id}/posts/{post_id}/comments",
    params(
        ("page" = i64, Query, description = "comment page number (they are splitted by 50)"),
        ("subicron_id" = String, Path, description = "Unique subicron ID"),
        ("post_id" = String, Path, description = "Unique post ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = GetSubicronIdPostsIdCommentsResDocs, example = json!({
            "status": "success",
            "data": [
                {
                    "text": "so cool!",
                    "embed_id": 14,
                    "commenter_id": 322,
                    "created_at": "date idk"
                },
                {
                    "text": "man this is fake i doont believe it",
                    "commenter_id": 555,
                    "created_at": "date idk"
                }
            ]
        })),
        (status = 401, description = "Unauthorized", body = GetSubicronIdPostsIdCommentsResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = GetSubicronIdPostsIdCommentsResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron Comments"
)]
#[get("/{subicron_id}/posts/{post_id}/comments")]
async fn get_subicron_id_posts_id_comments(
    token_data: HttpRequest,
    path: web::Path<(String, String)>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let page = query.page.unwrap_or(1);
        let subicron_id = parse_i64(path.0.clone(), "invalid subicron")?;
        let post_id = parse_i64(path.1.clone(), "invalid post_id")?;
        insure_subicron_exists(subicron_id).await?;
        insure_post_exists(subicron_id, post_id).await?;

        let comments = get_comments_on_post(post_id, page).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: Some(comments),
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: None,
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct GetSubicronIdPostsIdCommentsResDocs {
    status: &'static str,
    data: Option<Vec<CommentResDocs>>
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct CommentResDocs {
    pub text: String,
    pub embed_id: Option<i64>,
    pub commenter_id: i64,
    pub created_at: String
}