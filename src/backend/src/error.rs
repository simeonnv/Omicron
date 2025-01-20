use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use argon2::password_hash;

#[derive(Serialize, Deserialize)]
pub struct ErrorRes {
    status: String,
    data: &'static str
}

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorRes {
                status: msg.to_string(),
                data: ""
            }),
            Error::Internal(msg) => HttpResponse::InternalServerError().json(ErrorRes {
                status: format!("server skillissue: {}", msg),
                data: "",
            }),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Internal(format!("Database error: {}", err))
    }
}

impl From<password_hash::Error> for Error {
    fn from(err: password_hash::Error) -> Self {
        Error::Internal(format!("Crypto hash error: {}", err))
    }
}

