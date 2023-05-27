use crate::models::User;
use crate::schema::users::{self, email};
use crate::{auth, models::user::NewUser};
use diesel::{mysql::MysqlConnection, prelude::*, result::Error};
use nanoid::nanoid;

pub enum CreateUserError {
    PasswordError(bcrypt::BcryptError),
    DieselError(Error),
}

pub fn create_user(
    conn: &mut MysqlConnection,
    name: &str,
    user_email: &str,
    clear_password: &str,
) -> Result<User, CreateUserError> {
    let id = nanoid!(36);
    let hashed_password =
        auth::hash_password(clear_password).map_err(CreateUserError::PasswordError)?;
    let new_user = NewUser {
        id: id.as_str(),
        name,
        email: user_email,
        hashed_password: hashed_password.as_str(),
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .map_err(CreateUserError::DieselError)?;

    let created_user = users::table
        .filter(users::id.eq(id))
        .first(conn)
        .map_err(CreateUserError::DieselError)?;
    Ok(created_user)
}

pub fn get_user_by_email(conn: &mut MysqlConnection, user_email: &str) -> Result<User, Error> {
    let user = users::table.filter(email.eq(user_email)).first(conn)?;
    Ok(user)
}
