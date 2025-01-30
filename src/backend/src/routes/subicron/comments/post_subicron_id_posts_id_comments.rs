use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::insure_post_exists::insure_post_exists;
use crate::libs::auth::insure_subicron_exists::insure_subicron_exists;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::comments::create_comment_on_post::create_comment_on_post;
use crate::libs::comments::get_comments_on_post::Comment;


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<Vec<Comment>>
}

#[derive(Deserialize, Debug)]
struct Req {
    text: String,
    embed_id: Option<i64>
}

#[utoipa::path(
    post,
    path = "/subicron/{subicron_id}/posts/{post_id}/comments",
    request_body = PostSubicronIdPostsIdCommentsReqDocs,
    params(
        ("subicron_id" = String, Path, description = "Unique subicron ID"),
        ("post_id" = String, Path, description = "Unique post ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = PostSubicronIdPostsIdCommentsResDocs, example = json!({
            "status": "success",
            "data": ""
        })),
        (status = 401, description = "Unauthorized", body = PostSubicronIdPostsIdCommentsResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = PostSubicronIdPostsIdCommentsResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron Comments"
)]
#[post("/{subicron_id}/posts/{post_id}/comments")]
async fn post_subicron_id_posts_id_comments(
    token_data: HttpRequest,
    path: web::Path<(String, String)>,
    req: web::Json<Req>
) -> Result<HttpResponse, Error> {
    if let Some(account_data) = token_data.extensions().get::<AccountData>() {

        let subicron_id = parse_i64(path.0.clone(), "invalid subicron")?;
        let post_id = parse_i64(path.1.clone(), "invalid post_id")?;
        insure_subicron_exists(subicron_id).await?;
        insure_post_exists(subicron_id, post_id).await?;

        create_comment_on_post(post_id, account_data.id, req.text.clone(), req.embed_id).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: None,
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: None,
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct PostSubicronIdPostsIdCommentsResDocs {
    status: &'static str,
    data: &'static str
}

#[derive(Deserialize, ToSchema)]
#[allow(dead_code)]
struct PostSubicronIdPostsIdCommentsReqDocs {
    text: String,
    embed_id: Option<i64>
}