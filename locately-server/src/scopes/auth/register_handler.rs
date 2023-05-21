use std::fmt::Display;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    post, web, HttpResponse, Responder, Result,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError as ValidatorError, ValidationErrors};

use crate::{db::user_db::create_user, establish_connection};

#[derive(Debug, Validate, Deserialize)]
struct RegisterRequest {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8), custom = "validate_password_strength")]
    pub password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    username: String,
}

#[derive(Serialize, Debug, Error)]
struct UserError {
    message: String,
    detail: RegisterUserError,
}

#[post("/register")]
async fn register(req_body: web::Json<RegisterRequest>) -> Result<impl Responder, UserError> {
    req_body.validate().map_err(|e| UserError {
        message: "Validation failed".to_owned(),
        detail: RegisterUserError::ValidationError(e),
    })?;
    let mut conn = establish_connection();
    let user = create_user(
        &mut conn,
        &req_body.name,
        &req_body.email,
        &req_body.password,
    )
    .map_err(|e| UserError {
        message: "Failed to create user".to_owned(),
        detail: RegisterUserError::InternalError,
    })?;
    Ok(web::Json(user))
}

// === Error Handling ===

#[derive(Serialize, Debug, Display, Error)]
enum RegisterUserError {
    InternalError,
    ValidationError(ValidationErrors),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self.detail {
            RegisterUserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            RegisterUserError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

fn validate_password_strength(password: &str) -> Result<(), ValidatorError> {
    let contains_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let contains_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let contains_digit = password.chars().any(|c| c.is_ascii_digit());

    if contains_uppercase && contains_lowercase && contains_digit {
        Ok(())
    } else {
        Err(ValidatorError::new(
            "Password does not meet the required strength criteria.",
        ))
    }
}
