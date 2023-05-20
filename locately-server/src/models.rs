use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub hashed_password: bool,
}
