
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::libs::auth::insure_string_size::insure_string_size;
use crate::libs::auth::{check_if_account_exists::check_if_account_exists, create_account::create_account, create_token::create_token};
use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Req {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
struct Res<'a> {
    status: &'a str,
    data: String
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = PostAuthSignupDocReq,
    responses(
        (status = 200, description = "Signup successful", body = PostAuthSignupDocsRes, example = json!({
            "status": "success",
            "token": "abc123xyz456"
        })),
        (status = 401, description = "Unauthorized", body = PostAuthSignupDocsRes, example = json!({
            "status": "incorrect credential",
            "token": ""
        })),
        (status = 409, description = "Conflict", body = PostAuthSignupDocsRes, example = json!({
            "status": "account already exists",
            "token": ""
        }))
    ),
    security(),
    tag = "Auth"
)]
#[post("/signup")]
pub async fn signup(req: web::Json<Req>) -> Result<HttpResponse, Error> {

    insure_string_size(&req.username, 3, 15)?;
    insure_string_size(&req.password, 3, 30)?;
    
    if check_if_account_exists(&req.username).await? {
        return Ok(HttpResponse::Conflict().json(Res {
            status: "account already exists",
            data: "".to_string(),
        }))
    }

    let account: (String, i64) = create_account(&req.username, &req.password, "user", false).await?;


    let token: String = create_token(&account.1, "user".to_string()).await?;

    dbg!(&token);
    dbg!(&account);

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: token,
    }));
   
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Signup Request")]
pub struct PostAuthSignupDocReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(title = "Signup Response")]
struct PostAuthSignupDocsRes {
    status: String,
    token: String
}