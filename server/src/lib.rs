extern crate lazy_static;
extern crate diesel;
use diesel::{prelude::*, result::Error};

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

pub fn find_user_by_id(conn: &mut SqliteConnection, user_id: i32) -> Result<User, Error> {
    use schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .first::<User>(conn)
}