mod db;
mod config;
mod services;
mod repositories;
mod models;
mod controllers;

use actix_web::{web, App, HttpServer, middleware::Logger};
use log::info;

use crate::config::AppState;
use crate::controllers::UserController;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализация логгера
    std::env::set_var("RUST_LOG", "info,actix_web=debug");
    env_logger::init();
    
    info!("Запуск сервера...");
    
    // Инициализация подключения к базе данных
    let db = db::establish_connection()
        .await
        .expect("Ошибка при подключении к базе данных");
    
    info!("Подключение к базе данных установлено");
    
    // Инициализация таблиц
    db::init_db(&db)
        .await
        .expect("Ошибка при инициализации базы данных");
    
    info!("База данных инициализирована");

    // Создание состояния приложения
    let app_state = web::Data::new(AppState::new(db));

    info!("Сервер запущен на http://0.0.0.0:8080");

    let app = App::new()
        .app_data(app_state.clone())
        .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
        .route("/users", web::get().to(UserController::list_users))
        .route("/users", web::post().to(UserController::create_user))
        .route("/users/{id}", web::get().to(UserController::get_user))
        .route("/users/{id}", web::put().to(UserController::update_user))
        .route("/users/{id}", web::delete().to(UserController::delete_user));

    HttpServer::new(move || app)
        .bind("0.0.0.0:8080")?
        .run()
        .await
} 