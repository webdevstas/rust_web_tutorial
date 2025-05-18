use crate::models::Post;
use sea_orm::*;
use crate::models::post::{Entity as PostEntity, ActiveModel, Column};

pub struct PostRepository;

impl PostRepository {
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Post>, DbErr> {
        PostEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        title: String,
        content: String,
        user_id: i32,
    ) -> Result<Post, DbErr> {
        let post = ActiveModel {
            title: Set(title),
            content: Set(content),
            user_id: Set(user_id),
            ..Default::default()
        };

        post.insert(db).await
    }

    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<Post>, DbErr> {
        PostEntity::find()
            .filter(Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Post>, DbErr> {
        PostEntity::find().all(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Post, DbErr> {
        let mut post: ActiveModel = PostEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Post not found".to_string()))?
            .into();

        if let Some(title) = title {
            post.title = Set(title);
        }
        if let Some(content) = content {
            post.content = Set(content);
        }

        post.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
        let post = PostEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Post not found".to_string()))?;

        post.delete(db).await?;
        Ok(())
    }
} 