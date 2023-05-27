use crate::schema::users;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use serde::Serialize;

pub mod user;
#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Mysql))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub hashed_password: String,
}
