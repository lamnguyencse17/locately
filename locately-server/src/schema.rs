// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        hashed_password -> Varchar,
    }
}
