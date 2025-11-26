use actix_web::{
    HttpResponse, Result,
    web::{self, Json, post},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use db::{pool::DbPool, queries::auth};

use crate::utils::generate_jwt;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", post().to(register))
            .route("/login", post().to(login)),
    );
}

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

async fn register(pool: web::Data<DbPool>, body: Json<RegisterRequest>) -> Result<HttpResponse> {
    let username = body.username.trim();
    let password = body.password.trim();

    let hash_pass = hash(password, DEFAULT_COST).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Password hashing failed: {}", e))
    })?;

    let user = auth::create_user(&pool.0, username, &hash_pass)
        .await
        .map_err(|e| {
            let error_msg = e.to_string();
            if error_msg.contains("duplicate key") || error_msg.contains("unique constraint") {
                actix_web::error::ErrorBadRequest("Username already exists")
            } else {
                actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
            }
        })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": user.id,
        "username": user.username
    })))
}

async fn login(pool: web::Data<DbPool>, body: Json<LoginRequest>) -> Result<HttpResponse> {
    let user = auth::get_user_by_username(&pool.0, &body.username)
        .await
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid username or password"))?;

    let is_valid = verify(&body.password, &user.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification failed"))?;

    if !is_valid {
        return Err(actix_web::error::ErrorUnauthorized(
            "Invalid username or password",
        ));
    }

    let token = generate_jwt(user.id).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Token generation failed: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "user_id": user.id,
        "username": user.username
    })))
}
