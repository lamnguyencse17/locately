use diesel::prelude::*;
use serde::Serialize;

pub mod user;
#[derive(Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
}
