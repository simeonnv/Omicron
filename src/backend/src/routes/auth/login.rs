
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::libs::auth::check_credentials::check_credentials;
use crate::libs::auth::create_token::create_token;
use crate::error::Error;
use crate::libs::auth::insure_string_size::insure_string_size;

#[derive(Serialize, Deserialize)]
pub struct Req {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
struct Res<'a> {
    status: &'a str,
    data: Option<String>
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = PostAuthLoginDocReq,
    responses(
        (status = 200, description = "Signup successful", body = PostAuthLoginDocsRes, example = json!({
            "status": "success",
            "token": "abc123xyz456"
        })),
        (status = 401, description = "Unauthorized", body = PostAuthLoginDocsRes, example = json!({
            "status": "incorrect credential",
            "token": ""
        })),
        (status = 409, description = "Conflict", body = PostAuthLoginDocsRes, example = json!({
            "status": "account already exists",
            "token": ""
        }))
    ),
    security(),
    tag = "Auth"
)]
#[post("/login")]
pub async fn login(req: web::Json<Req>) -> Result<HttpResponse, Error> {

    insure_string_size(&req.username, 3, 15)?;
    insure_string_size(&req.password, 3, 30)?;

    let account = check_credentials(&req.username, &req.password).await?;
    if !account.0 {
        return Ok(HttpResponse::Conflict().json(Res {
            status: "incorrect credentials",
            data: None,
        }))
    }

    let token: String = create_token(&account.1, account.2).await?;

    dbg!(&token);

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: Some(token),
    }));
   
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Login Request")]
pub struct PostAuthLoginDocReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(title = "Login Response")]
struct PostAuthLoginDocsRes {
    status: String,
    token: String
}