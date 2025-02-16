use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::insure_post_exists::insure_post_exists;
use crate::libs::auth::insure_subicron_exists::insure_subicron_exists;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::posts::get_upvotes::{get_upvotes, GetUpvotesRes};


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<GetUpvotesRes>
}

#[utoipa::path(
    get,
    path = "/subicron/{subicron_id}/posts/{post_id}/upvote",
    params(
        ("subicron_id" = String, Path, description = "Unique subicron ID"),
        ("post_id" = String, Path, description = "Unique post ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = GetSubicronIdPostsIdUpvoteResDocs, example = json!({
            "status": "success",
            "data": {
                "upvotes": 6222,
                "is_upvoted": true
            }
        })),
        (status = 400, description = "Bad Request", body = GetSubicronIdPostsIdUpvoteResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron Posts"
)]
#[get("/{subicron_id}/posts/{post_id}/upvote")]
async fn get_subicron_id_posts_id_upvote(
    token_data: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    if let Some(account_data) = token_data.extensions().get::<AccountData>() {

        let subicron_id = parse_i64(path.0.clone(), "invalid subicron")?;
        let post_id = parse_i64(path.1.clone(), "invalid post_id")?;
        insure_subicron_exists(subicron_id).await?;
        insure_post_exists(subicron_id, post_id).await?;

        //insuring that post exists (very austistic)
        let post = get_upvotes(account_data.id, post_id).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "Success",
            data: Some(post),
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: None,
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct GetSubicronIdPostsIdUpvoteResDocs {
    status: &'static str,
    data: Option<GetUpvotesRes>
}