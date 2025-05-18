use crate::models::Comment;
use sea_orm::*;
use crate::models::comment::{Entity as CommentEntity, ActiveModel, Column};

pub struct CommentRepository;

impl CommentRepository {
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Comment>, DbErr> {
        CommentEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        content: String,
        user_id: i32,
        post_id: i32,
    ) -> Result<Comment, DbErr> {
        let comment = ActiveModel {
            content: Set(content),
            user_id: Set(user_id),
            post_id: Set(post_id),
            ..Default::default()
        };

        comment.insert(db).await
    }

    pub async fn find_by_post_id(
        db: &DatabaseConnection,
        post_id: i32,
    ) -> Result<Vec<Comment>, DbErr> {
        CommentEntity::find()
            .filter(Column::PostId.eq(post_id))
            .all(db)
            .await
    }

    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<Comment>, DbErr> {
        CommentEntity::find()
            .filter(Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        content: Option<String>,
    ) -> Result<Comment, DbErr> {
        let mut comment: ActiveModel = CommentEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Comment not found".to_string()))?
            .into();

        if let Some(content) = content {
            comment.content = Set(content);
        }

        comment.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
        let comment = CommentEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Comment not found".to_string()))?;

        comment.delete(db).await?;
        Ok(())
    }
} 