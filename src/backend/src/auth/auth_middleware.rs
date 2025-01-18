use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ResponseError,
    Error, HttpMessage, HttpResponse,
};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Error as SqlxError;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::fmt;

use crate::db::get_db_pool::get_db_pool;

#[derive(Debug)]
pub struct AuthError(pub String);

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(ErrorRes {
            status: self.0.clone(),
            data: "",
        })
    }
}

impl From<SqlxError> for AuthError {
    fn from(err: SqlxError) -> Self {
        AuthError(format!("Database error: {}", err))
    }
}

#[derive(serde::Serialize)]
struct ErrorRes {
    status: String,
    data: &'static str,
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .map(|auth| auth.to_string());

            if let Some(auth_header) = auth_header {
                if auth_header.starts_with("Bearer ") {
                    let token = String::from(&auth_header["Bearer ".len()..]);
                    let pool = get_db_pool();

                    #[derive(sqlx::FromRow)]
                    
                    struct AccountData {
                        token: String,
                        role: String,
                        token_creation_date: NaiveDateTime,
                        username: String,
                        account_creation_date: NaiveDateTime,
                    }

                    let db_res: Option<AccountData> = sqlx::query_as(r#"
                        SELECT 
                            Tokens.token, Tokens.role, Tokens.created_at AS token_creation_date,
                            Accounts.username, Accounts.created_at AS account_creation_date
                        FROM Tokens
                        INNER JOIN Accounts ON Tokens.token_id = Accounts.account_id
                        WHERE token = ?;
                    "#)
                        .bind(token)
                        .fetch_optional(pool)
                        .await
                        .map_err(AuthError::from)?; // Use custom error conversion

                    let account_data = db_res.ok_or_else(|| AuthError("Token has expired!".to_string()))?;

                    req.extensions_mut().insert(account_data);
                    return service.call(req).await;
                }
            }

            Err(AuthError("Invalid or missing token".to_string()).into())
        })
    }
}
