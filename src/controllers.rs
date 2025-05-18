use actix_web::{web, HttpResponse, Responder};
use log::{info, error};
use serde::Deserialize;

use crate::config::AppState;
use crate::services::UserService;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct CreateUserRequest {
    name: String,
    age: i32,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct UpdateUserRequest {
    name: Option<String>,
    age: Option<i32>,
}

pub struct UserController;

impl UserController {
    pub async fn list_users(
        state: web::Data<AppState>,
    ) -> impl Responder {
        info!("Получен запрос на список пользователей");
        let user_service = UserService::new((*state.db).clone());
        match user_service.get_all_users().await {
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

    pub async fn get_user(
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

    pub async fn create_user(
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

    pub async fn update_user(
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

    pub async fn delete_user(
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
} 