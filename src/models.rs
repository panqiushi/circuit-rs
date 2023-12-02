use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub hash_password: String,
}