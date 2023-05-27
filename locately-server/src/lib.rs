use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};
use diesel::mysql::MysqlConnection;
use diesel::r2d2;
use serde::Serialize;
use std::fmt::Display;
use validator::ValidationErrors;

pub mod auth;
pub mod db;
pub mod models;
pub mod schema;
pub mod scopes;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

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
