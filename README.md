# Учебный веб-сервер на Rust

Этот проект демонстрирует основные концепции языка Rust на примере простого веб-сервера с поддержкой PostgreSQL.

## Основные концепции Rust, представленные в проекте

### 1. Структуры и атрибуты
```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub created_at: DateTimeWithTimeZone,
}
```
- `#[derive(...)]` - это атрибут, который автоматически реализует трейты для структуры
- Структуры в Rust похожи на интерфейсы в TypeScript, но с более строгой типизацией
- Атрибуты в Rust похожи на декораторы в TypeScript, но более мощные

### 2. Асинхронное программирование
```rust
async fn create_user(
    db: &DatabaseConnection,
    name: String,
    age: i32,
) -> Result<UserModel, DbErr> {
    // ...
}
```
- `async/await` в Rust работает аналогично TypeScript
- `Result<T, E>` - это тип для обработки ошибок, похожий на try/catch в TypeScript
- Асинхронные функции могут возвращать `Result` для обработки ошибок

### 3. Модульная система
```rust
mod models;
mod db;
mod crud;
```
- Модули в Rust помогают организовать код
- Похоже на систему модулей в TypeScript, но с более строгими правилами видимости

### 4. Работа с базой данных
```rust
use sea_orm::*;
```
- SeaORM - это ORM для Rust, похожий на TypeORM в TypeScript
- Поддерживает асинхронные операции
- Имеет встроенную поддержку миграций

## Запуск проекта

### Вариант 1: Локальный запуск

1. Убедитесь, что у вас установлен Rust:
   ```bash
   rustc --version
   ```

2. Установите PostgreSQL, если еще не установлен

3. Создайте базу данных:
   ```sql
   CREATE DATABASE rust_tutorial;
   ```

4. Создайте файл `.env` в корне проекта:
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/rust_tutorial
   ```
   Замените параметры подключения на ваши.

5. Запустите проект:
   ```bash
   cargo run
   ```

### Вариант 2: Запуск через Docker

1. Убедитесь, что у вас установлены Docker и Docker Compose:
   ```bash
   docker --version
   docker-compose --version
   ```

2. Запустите проект:
   ```bash
   docker-compose up --build
   ```

3. Для остановки:
   ```bash
   docker-compose down
   ```

4. Для удаления всех данных (включая базу данных):
   ```bash
   docker-compose down -v
   ```

## Доступные эндпоинты

1. GET `/users` - Получить список всех пользователей
2. GET `/users/{id}` - Получить пользователя по ID
3. POST `/users` - Создать нового пользователя
4. PUT `/users/{id}` - Обновить пользователя
5. DELETE `/users/{id}` - Удалить пользователя

## Примеры запросов

### Создание пользователя (POST /users)
```bash
curl -X POST -H "Content-Type: application/json" \
     -d '{"name":"Иван","age":25}' \
     http://localhost:8080/users
```

### Получение пользователя по ID (GET /users/{id})
```bash
curl http://localhost:8080/users/1
```

### Обновление пользователя (PUT /users/{id})
```bash
curl -X PUT -H "Content-Type: application/json" \
     -d '{"name":"Иван Иванов","age":26}' \
     http://localhost:8080/users/1
```

### Удаление пользователя (DELETE /users/{id})
```bash
curl -X DELETE http://localhost:8080/users/1
```

## Основные отличия от TypeScript

1. **Система типов**
   - Rust имеет более строгую систему типов
   - Нет `null` или `undefined`, вместо этого используется `Option<T>`
   - Все переменные по умолчанию иммутабельны

2. **Владение (Ownership)**
   - Уникальная концепция Rust
   - Каждое значение имеет только одного владельца
   - Помогает предотвратить утечки памяти и гонки данных

3. **Макросы**
   - Более мощные чем декораторы в TypeScript
   - Используются для генерации кода во время компиляции

4. **Обработка ошибок**
   - Использует `Result<T, E>` вместо try/catch
   - Более явная обработка ошибок

5. **Работа с базой данных**
   - SeaORM предоставляет типобезопасный API
   - Все операции с базой данных асинхронные
   - Встроенная поддержка миграций 