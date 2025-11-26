use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::user::User;

pub async fn create_user(
    pool: &Pool<Postgres>,
    username: &str,
    password: &str,
) -> sqlx::Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *",
    )
    .bind(username)
    .bind(password)
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_username(pool: &Pool<Postgres>, username: &str) -> sqlx::Result<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: Uuid) -> sqlx::Result<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
}
