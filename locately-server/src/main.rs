use actix_web::{App, HttpServer};
use locately_server::scopes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(scopes::auth_scope_config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
