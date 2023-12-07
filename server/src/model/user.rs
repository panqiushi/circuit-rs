use sea_orm::{entity::prelude::*, Database};
use serde::{Deserialize, Serialize};

use crate::model::db::DB;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub hash_password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


pub async fn insert_user(user: &Model) -> Result<Model, DbErr> {
    let user = ActiveModel::from(user.clone());
    let result = user.insert(&*DB).await?;
    Ok(result)
}