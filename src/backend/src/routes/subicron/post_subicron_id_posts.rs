use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::insure_file_exists::insure_file_exists;
use crate::libs::auth::insure_string_size::insure_string_size;
use crate::libs::auth::insure_subicron_exists::insure_subicron_exists;
use crate::libs::subicron::create_post::create_post;


#[derive(Serialize, Deserialize, Debug)]
struct Res {
    status: &'static str,
    data: &'static str
}


#[derive(Serialize, Deserialize)]
pub struct Req {
    pub header: String,
    pub body: String,
    pub embed: Option<i64>    
}


#[utoipa::path(
    post,
    path = "/subicron/{subicron_id}/posts",
    request_body = PostSubicronReqDocs,
    responses(
        (status = 200, description = "Signup successful", body = PostSubicronResDocs, example = json!({
            "status": "success",
            "data": ""
        })),
        (status = 401, description = "Unauthorized", body = PostSubicronResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = PostSubicronResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Subicron"
)]
#[post("/{subicron_id}/posts")]
async fn post_subicron_id_posts(token_data: HttpRequest, req: web::Json<Req>, path: web::Path<String>) -> Result<HttpResponse, Error> {
    if let Some(account_info) = token_data.extensions().get::<AccountData>() {

        let subicron_id = path.parse::<i64>().map_err(|_| {
            Error::BadRequest(format!("invalid subicron id: {}", path))
        })?;
        
        // insure everything is up to the hespotos hermanus standarts
        insure_string_size(&req.header, 5, 20)?;
        insure_string_size(&req.body, 5, 510)?;
        insure_subicron_exists(subicron_id).await?;
        if !req.embed.is_none() {
            insure_file_exists(req.embed.unwrap_or(1)).await?;
        }

        create_post(&req.header, &req.body, req.embed, account_info.id, subicron_id).await?;

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


#[derive(Serialize, Deserialize, ToSchema)]
struct PostSubicronResDocs {
    status: &'static str,
    data: &'static str
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct PostSubicronReqDocs {
    pub name: String,
    pub image_id: Option<i64>,
}
