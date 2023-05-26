use actix_web::{post, web, Responder};
use serde::Deserialize;
use validator::Validate;

use crate::{
    auth::is_password_matched, db::user_db::get_user_by_email, establish_connection, ErrorEnum,
    ErrorResponse,
};

#[derive(Debug, Validate, Deserialize)]
struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[post("/login")]
async fn login(req_body: web::Json<LoginRequest>) -> Result<impl Responder, ErrorResponse> {
    let mut conn = establish_connection();
    let found_user =
        get_user_by_email(&mut conn, req_body.email.as_str()).map_err(|_| ErrorResponse {
            message: "Email or password is invalid".to_owned(),
            detail: ErrorEnum::AuthorizationError,
        })?;
    let password_matched = is_password_matched(&req_body.password, &found_user.hashed_password);
    if !password_matched {
        return Err(ErrorResponse {
            message: "Email or password is invalid".to_owned(),
            detail: ErrorEnum::AuthorizationError,
        });
    }
    Ok(web::Json(()))
}
