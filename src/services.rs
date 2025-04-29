use sea_orm::*;
use crate::models::{self, Entity as User, Model as UserModel};

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(
        &self,
        name: String,
        age: i32,
    ) -> Result<UserModel, DbErr> {
        let user = models::ActiveModel {
            name: Set(name),
            age: Set(age),
            ..Default::default()
        };
        
        user.insert(&self.db).await
    }

    pub async fn get_user(
        &self,
        id: i32,
    ) -> Result<Option<UserModel>, DbErr> {
        User::find_by_id(id).one(&self.db).await
    }

    pub async fn update_user(
        &self,
        id: i32,
        name: Option<String>,
        age: Option<i32>,
    ) -> Result<Option<UserModel>, DbErr> {
        let user: Option<models::ActiveModel> = User::find_by_id(id)
            .one(&self.db)
            .await?
            .map(Into::into);

        if let Some(mut user) = user {
            if let Some(name) = name {
                user.name = Set(name);
            }
            if let Some(age) = age {
                user.age = Set(age);
            }
            Ok(Some(user.update(&self.db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_user(
        &self,
        id: i32,
    ) -> Result<bool, DbErr> {
        let user = User::find_by_id(id).one(&self.db).await?;
        if let Some(user) = user {
            user.delete(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn list_users(
        &self,
    ) -> Result<Vec<UserModel>, DbErr> {
        User::find().all(&self.db).await
    }
} 