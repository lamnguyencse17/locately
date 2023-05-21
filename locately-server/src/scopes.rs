use actix_web::web;

mod auth;

pub fn auth_scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth::register));
}
