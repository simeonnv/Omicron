use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;


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
async fn post_subicron_id_posts(token_data: HttpRequest, req: web::Json<Req>) -> Result<HttpResponse, Error> {
    if let Some(account_info) = token_data.extensions().get::<AccountData>() {

        

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
