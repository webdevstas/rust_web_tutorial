mod models;
mod db;
mod config;
mod services;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::Deserialize;
use log::{info, error};

use crate::config::AppState;
use crate::services::UserService;

#[derive(Deserialize)]
#[derive(Debug)]
struct CreateUserRequest {
    name: String,
    age: i32,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct UpdateUserRequest {
    name: Option<String>,
    age: Option<i32>,
}

// Обработчик GET запроса для получения списка пользователей
async fn list_users(
    state: web::Data<AppState>,
) -> impl Responder {
    info!("Получен запрос на список пользователей");
    let user_service = UserService::new((*state.db).clone());
    match user_service.list_users().await {
        Ok(users) => {
            info!("Успешно получен список пользователей: {} записей", users.len());
            HttpResponse::Ok().json(users)
        },
        Err(e) => {
            error!("Ошибка при получении пользователей: {}", e);
            HttpResponse::InternalServerError().body("Ошибка при получении пользователей")
        },
    }
}

// Обработчик GET запроса для получения пользователя по ID
async fn get_user(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    info!("Получен запрос на получение пользователя с ID: {}", user_id);
    let user_service = UserService::new((*state.db).clone());
    match user_service.get_user(user_id).await {
        Ok(Some(user)) => {
            info!("Пользователь с ID {} успешно найден", user_id);
            HttpResponse::Ok().json(user)
        },
        Ok(None) => {
            info!("Пользователь с ID {} не найден", user_id);
            HttpResponse::NotFound().body("Пользователь не найден")
        },
        Err(e) => {
            error!("Ошибка при получении пользователя с ID {}: {}", user_id, e);
            HttpResponse::InternalServerError().body("Ошибка при получении пользователя")
        },
    }
}

// Обработчик POST запроса для создания пользователя
async fn create_user(
    state: web::Data<AppState>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    info!("Получен запрос на создание пользователя: {:?}", user);
    let user_service = UserService::new((*state.db).clone());
    match user_service.create_user(user.name.clone(), user.age).await {
        Ok(user) => {
            info!("Пользователь успешно создан с ID: {}", user.id);
            HttpResponse::Created().json(user)
        },
        Err(e) => {
            error!("Ошибка при создании пользователя: {}", e);
            HttpResponse::InternalServerError().body("Ошибка при создании пользователя")
        },
    }
}

// Обработчик PUT запроса для обновления пользователя
async fn update_user(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    user: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let user_id = path.into_inner();
    info!("Получен запрос на обновление пользователя с ID {}: {:?}", user_id, user);
    let user_service = UserService::new((*state.db).clone());
    match user_service.update_user(user_id, user.name.clone(), user.age).await {
        Ok(Some(user)) => {
            info!("Пользователь с ID {} успешно обновлен", user_id);
            HttpResponse::Ok().json(user)
        },
        Ok(None) => {
            info!("Пользователь с ID {} не найден при попытке обновления", user_id);
            HttpResponse::NotFound().body("Пользователь не найден")
        },
        Err(e) => {
            error!("Ошибка при обновлении пользователя с ID {}: {}", user_id, e);
            HttpResponse::InternalServerError().body("Ошибка при обновлении пользователя")
        },
    }
}

// Обработчик DELETE запроса для удаления пользователя
async fn delete_user(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    info!("Получен запрос на удаление пользователя с ID: {}", user_id);
    let user_service = UserService::new((*state.db).clone());
    match user_service.delete_user(user_id).await {
        Ok(true) => {
            info!("Пользователь с ID {} успешно удален", user_id);
            HttpResponse::NoContent().finish()
        },
        Ok(false) => {
            info!("Пользователь с ID {} не найден при попытке удаления", user_id);
            HttpResponse::NotFound().body("Пользователь не найден")
        },
        Err(e) => {
            error!("Ошибка при удалении пользователя с ID {}: {}", user_id, e);
            HttpResponse::InternalServerError().body("Ошибка при удалении пользователя")
        },
    }
}

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
            .route("/users", web::get().to(list_users))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
} 