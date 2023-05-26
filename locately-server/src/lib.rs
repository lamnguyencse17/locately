use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::Serialize;
use std::env;
use std::fmt::Display;
use validator::ValidationErrors;

pub mod auth;
pub mod db;
pub mod models;
pub mod schema;
pub mod scopes;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Debug, Display, Error)]
pub enum ErrorEnum {
    InternalError,
    ValidationError(ValidationErrors),
    AuthorizationError,
}

#[derive(Serialize, Debug, Error)]
pub struct ErrorResponse {
    message: String,
    detail: ErrorEnum,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl error::ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self.detail {
            ErrorEnum::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorEnum::ValidationError(_) => StatusCode::BAD_REQUEST,
            ErrorEnum::AuthorizationError => StatusCode::UNAUTHORIZED,
        }
    }
}
