use sea_orm::*;
use crate::models::{self, Entity as User, Model as UserModel};

pub async fn create_user(
    db: &DatabaseConnection,
    name: String,
    age: i32,
) -> Result<UserModel, DbErr> {
    let user = models::ActiveModel {
        name: Set(name),
        age: Set(age),
        ..Default::default()
    };
    
    user.insert(db).await
}

pub async fn get_user(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<UserModel>, DbErr> {
    User::find_by_id(id).one(db).await
}

pub async fn update_user(
    db: &DatabaseConnection,
    id: i32,
    name: Option<String>,
    age: Option<i32>,
) -> Result<Option<UserModel>, DbErr> {
    let user: Option<models::ActiveModel> = User::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into);

    if let Some(mut user) = user {
        if let Some(name) = name {
            user.name = Set(name);
        }
        if let Some(age) = age {
            user.age = Set(age);
        }
        Ok(Some(user.update(db).await?))
    } else {
        Ok(None)
    }
}

pub async fn delete_user(
    db: &DatabaseConnection,
    id: i32,
) -> Result<bool, DbErr> {
    let user = User::find_by_id(id).one(db).await?;
    if let Some(user) = user {
        user.delete(db).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn list_users(
    db: &DatabaseConnection,
) -> Result<Vec<UserModel>, DbErr> {
    User::find().all(db).await
} 