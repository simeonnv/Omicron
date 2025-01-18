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
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(ErrorRes {
            status: format!("server skillissue: {}", self.0),
            data: "",
        })
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error(format!("Database error: {}", err))
    }
}

impl From<password_hash::Error> for Error {
    fn from(err: password_hash::Error) -> Self {
        Error(format!("Password hash error: {}", err))
    }
}
