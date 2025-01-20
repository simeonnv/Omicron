use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::auth::insure_string_size::insure_string_size;
use crate::libs::subicron::create_subicron::create_subicron;

#[derive(Serialize, Deserialize, Debug)]
struct Res {
    status: &'static str,
    data: &'static str
}


#[derive(Serialize, Deserialize)]
pub struct Req {
    pub name: String,
    pub image_id: Option<i64>,
}


#[utoipa::path(
    post,
    path = "/subicron",
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
#[post("")]
async fn post_subicron(token_data: HttpRequest, req: web::Json<Req>) -> Result<HttpResponse, Error> {
    if let Some(account_info) = token_data.extensions().get::<AccountData>() {
        if account_info.role != "admin" {
            return Ok(HttpResponse::Unauthorized().json(Res {
                status: "Invalid premisions",
                data: "",
            }))
        }

        insure_string_size(&req.name, 3, 15)?;

        let subicron_data = create_subicron(&req.name, req.image_id).await?;
        
        dbg!(&subicron_data);

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
