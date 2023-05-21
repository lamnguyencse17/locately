use crate::schema::users;
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub hashed_password: &'a str,
}
