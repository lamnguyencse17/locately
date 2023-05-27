use actix_web::{post, web, Responder};
use serde::Deserialize;
use validator::Validate;

use crate::{
    auth::is_password_matched, db::user_db::get_user_by_email, DbPool, ErrorEnum, ErrorResponse,
};

#[derive(Debug, Validate, Deserialize)]
struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    req_body: web::Json<LoginRequest>,
) -> Result<impl Responder, ErrorResponse> {
    let user_email = req_body.email.clone();

    let found_user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        get_user_by_email(&mut conn, user_email.as_str())
    })
    .await
    .map_err(|_| ErrorResponse {
        message: "Failed to authenticate user".to_owned(),
        detail: ErrorEnum::InternalError,
    })?
    .map_err(|_| ErrorResponse {
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
