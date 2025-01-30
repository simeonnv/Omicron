use actix_web::{delete, web, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::parse_i64::parse_i64;
use crate::libs::posts::delete_upvote_post::delete_upvote_post;
use crate::libs::posts::get_post_from_id::get_post_from_id;


#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: &'static str
}

#[utoipa::path(
    delete,
    path = "/subicron/{subicron_id}/posts/{post_id}/upvote",
    params(
        ("subicron_id" = String, Path, description = "Unique subicron ID"),
        ("post_id" = String, Path, description = "Unique post ID")
    ),
    responses(
        (status = 200, description = "Signup successful", body = DeleteSubicronIdPostsIdUpvoteResDocs, example = json!({
            "status": "success",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = DeleteSubicronIdPostsIdUpvoteResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron Posts"
)]
#[delete("/{subicron_id}/posts/{post_id}/upvote")]
async fn delete_subicron_id_posts_id_upvote(
    token_data: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    if let Some(account_data) = token_data.extensions().get::<AccountData>() {

        let subicron_id = parse_i64(path.0.clone(), "invalid subicron")?;
        let post_id = parse_i64(path.1.clone(), "invalid post_id")?;

        //insuring that post exists (very austistic)
        let post = get_post_from_id(post_id, subicron_id).await?;

        delete_upvote_post(post.post_id, account_data.id).await?;

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
struct DeleteSubicronIdPostsIdUpvoteResDocs {
    status: &'static str,
    data: &'static str
}