use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    #[sea_orm(default = "now")]
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::user::Entity",
    //     from = "Column::UserId",
    //     to = "super::user::Column::Id"
    // )]
    // User,
    // #[sea_orm(has_many = "super::comment::Entity")]
    // Comments,
}

impl ActiveModelBehavior for ActiveModel {} 