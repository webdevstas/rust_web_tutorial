use sea_orm::*;
use dotenv::dotenv;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    Database::connect(&database_url).await
}

pub async fn init_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Создаем таблицу users, если она не существует
    let sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            age INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    "#;
    
    db.execute_unprepared(sql).await?;
    Ok(())
} 