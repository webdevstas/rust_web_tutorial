use sea_orm::DatabaseConnection;
use std::sync::Arc;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub host: String,
    pub port: String,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db),
            host: env::var("HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| 8080.to_string())
        }
    }
} 