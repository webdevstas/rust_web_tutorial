mod db;
mod config;
mod services;
mod repositories;
mod models;
mod controllers;

use actix_web::{web, App, HttpServer, middleware::Logger};
use log::info;

use crate::config::AppState;
use crate::controllers::config_user_controller;

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

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .configure(config_user_controller)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
} 