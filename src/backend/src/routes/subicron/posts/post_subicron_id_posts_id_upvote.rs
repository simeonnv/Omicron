use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::insure_post_exists::insure_post_exists;
use crate::libs::auth::insure_subicron_exists::insure_subicron_exists;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::posts::get_post_from_id::get_post_from_id;
use crate::libs::posts::upvote_post::upvote_post;


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: &'static str
}

#[utoipa::path(
    post,
    path = "/subicron/{subicron_id}/posts/{post_id}/upvote",
    params(
        ("subicron_id" = String, Path, description = "Unique subicron ID"),
        ("post_id" = String, Path, description = "Unique post ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = PostSubicronIdPostsIdUpvoteResDocs, example = json!({
            "status": "success",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = PostSubicronIdPostsIdUpvoteResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron Posts"
)]
#[post("/{subicron_id}/posts/{post_id}/upvote")]
async fn post_subicron_id_posts_id_upvote(
    token_data: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    if let Some(account_data) = token_data.extensions().get::<AccountData>() {

        let subicron_id = parse_i64(path.0.clone(), "invalid subicron")?;
        let post_id = parse_i64(path.1.clone(), "invalid post_id")?;
        insure_subicron_exists(subicron_id).await?;
        insure_post_exists(subicron_id, post_id).await?;

        //insuring that post exists (very austistic)
        let post = get_post_from_id(post_id, subicron_id).await?;

        upvote_post(post.post_id,account_data.id).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: "",
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: "",
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct PostSubicronIdPostsIdUpvoteResDocs {
    status: &'static str,
    data: &'static str
}