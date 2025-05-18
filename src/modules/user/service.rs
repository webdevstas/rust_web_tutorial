use crate::modules::user::repository::UserRepository;
use crate::modules::user::User;
use sea_orm::*;
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {  
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<User>, DbErr> {
        UserRepository::find_by_id(&self.db, id).await
    }

    pub async fn create_user(
        &self,
        name: String,
        age: i32,
    ) -> Result<User, DbErr> {
        UserRepository::create(&self.db, name, age).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, DbErr> {
        UserRepository::find_all(&self.db).await
    }

    pub async fn update_user(
        &self,
        id: i32,
        name: Option<String>,
        age: Option<i32>,
    ) -> Result<Option<User>, DbErr> {
        UserRepository::update(&self.db, id, name, age).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<bool, DbErr> {
        UserRepository::delete(&self.db, id).await
    }
} 