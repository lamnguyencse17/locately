use crate::{auth, models::user::NewUser};
// use crate::models::user::NewUser;
use crate::models::User;
use crate::schema::users;
use diesel::{mysql::MysqlConnection, prelude::*, result::Error};
use nanoid::nanoid;

pub enum CreateUserError {
    PasswordError(bcrypt::BcryptError),
    DieselError(Error),
}

pub fn create_user(
    conn: &mut MysqlConnection,
    name: &str,
    email: &str,
    clear_password: &str,
) -> Result<User, CreateUserError> {
    let id = nanoid!(36);
    let hashed_password =
        auth::hash_password(clear_password).map_err(|e| CreateUserError::PasswordError(e))?;
    let new_user = NewUser {
        id: id.as_str(),
        name,
        email,
        hashed_password: hashed_password.as_str(),
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .map_err(|e| CreateUserError::DieselError(e))?;

    let created_user = users::table
        .filter(users::id.eq(id))
        .first(conn)
        .map_err(|e| CreateUserError::DieselError(e))?;
    return Ok(created_user);
}
