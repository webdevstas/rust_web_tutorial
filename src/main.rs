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
    let host = &app_state.host;
    let port = &app_state.port;
    let url = format!("{host}:{port}");

    info!("Сервер запущен на http://{url}");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .configure(config_user_controller)
    })
    .bind(url)?
    .run()
    .await
} 