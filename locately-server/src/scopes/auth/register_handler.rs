use actix_web::{post, web, Responder, Result};
use serde::Deserialize;
use validator::Validate;

use crate::{
    auth::validate_password_strength, db::user_db::create_user, DbPool, ErrorEnum, ErrorResponse,
};

#[derive(Debug, Validate, Deserialize)]
struct RegisterRequest {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8), custom = "validate_password_strength")]
    pub password: String,
}

#[post("/register")]
async fn register(
    pool: web::Data<DbPool>,
    req_body: web::Json<RegisterRequest>,
) -> Result<impl Responder, ErrorResponse> {
    req_body.validate().map_err(|e| ErrorResponse {
        message: "Validation failed".to_owned(),
        detail: ErrorEnum::ValidationError(e),
    })?;
    let user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        create_user(
            &mut conn,
            &req_body.name,
            &req_body.email,
            &req_body.password,
        )
    })
    .await
    .map_err(|_| ErrorResponse {
        message: "Failed to create user".to_owned(),
        detail: ErrorEnum::InternalError,
    })?
    .map_err(|_| ErrorResponse {
        message: "Failed to create user".to_owned(),
        detail: ErrorEnum::InternalError,
    })?;
    Ok(web::Json(user))
}
