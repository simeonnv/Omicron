
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::auth::{check_if_account_exists::check_if_account_exists, create_account::create_account, create_token::create_token};
use crate::error::Error;

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
pub async fn signup(data: web::Json<Req>) -> Result<HttpResponse, Error> {

    if data.username.len() > 15 || data.username.len() < 3 || data.password.len() > 30 || data.password.len() < 3 {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "incorrect credentials",
            data: None,
        }));
    }
    
    if check_if_account_exists(&data.username).await? {
        return Ok(HttpResponse::Conflict().json(Res {
            status: "account already exists",
            data: None,
        }))
    }

    let account: (String, i64) = create_account(&data.username, &data.password, "user").await?;


    let token: String = create_token(&account.1, "user").await?;

    dbg!(&token);
    dbg!(&account);

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: Some(token),
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