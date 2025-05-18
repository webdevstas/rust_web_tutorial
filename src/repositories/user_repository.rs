use crate::models::User;
use sea_orm::*;
use crate::models::user::{Entity as UserEntity, ActiveModel};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<User>, DbErr> {
        UserEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        age: i32,
    ) -> Result<User, DbErr> {
        let user = ActiveModel {
            name: Set(name),
            age: Set(age),
            ..Default::default()
        };

        user.insert(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<User>, DbErr> {
        UserEntity::find().all(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        name: Option<String>,
        age: Option<i32>,
    ) -> Result<Option<User>, DbErr> {
        let mut user: ActiveModel = UserEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?
            .into();

        if let Some(name) = name {
            user.name = Set(name);
        }
        if let Some(age) = age {
            user.age = Set(age);
        }

        if let Ok(user) = user.update(db).await {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool, DbErr> {
        let user = UserEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        user.delete(db).await?;
        Ok(true)
    }
} 