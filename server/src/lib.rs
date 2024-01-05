extern crate lazy_static;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::prelude::*;

use dotenvy::dotenv;
use std::env;

use crate::models::User;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut SqliteConnection, username: &str, email: &str) -> User {
    use schema::users;

    let new_user = User {
        username:username.to_string(), 
        email: email.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    users::table
        .order(users::id.desc())
        .first(conn)
        .expect("Error saving new user")
}