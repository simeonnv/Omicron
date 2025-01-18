use std::error::Error;

use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{db::get_db_pool::get_db_pool, DB};

#[derive(Serialize, Deserialize)]
pub struct Req {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
struct Res<'a> {
    status: &'a str,
    token: String
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = AuthSignupDocReq,
    responses(
        (status = 200, description = "Signup successful", body = AuthSignupDocsRes, example = json!({
            "status": "success",
            "token": "abc123xyz456"
        })),
        (status = 401, description = "Unauthorized", body = AuthSignupDocsRes, example = json!({
            "status": "incorrect credential",
            "token": ""
        })),
        (status = 409, description = "Conflict", body = AuthSignupDocsRes, example = json!({
            "status": "account already exists",
            "token": ""
        }))
    ),
    security(),
    tag = "Auth"
)]
#[post("/signup")]
pub async fn signup(data: web::Json<Req>) -> Result<HttpResponse, Box<dyn Error>> {

    if data.username.len() > 15 || data.username.len() < 3 || data.password.len() > 30 || data.password.len() < 3 {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "incorrect credentials",
            token: "".to_string(),
        }));
    }

    let pool = get_db_pool();

    




    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        token: "".to_string(),
    }));
   
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Signup Request")]
pub struct AuthSignupDocReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(title = "Signup Response")]
struct AuthSignupDocsRes {
    status: String,
    token: String
}