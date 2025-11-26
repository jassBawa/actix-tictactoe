use actix_web::{HttpResponse, Result, web};
use db::pool::DbPool;
use uuid::Uuid;

use crate::middleware::{AuthenticatedUser, jwt_auth_fn};
use actix_web_lab::middleware::from_fn;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/game")
            .wrap(from_fn(jwt_auth_fn))
            .route("/create", web::post().to(create_game))
            .route("/join", web::post().to(join_game))
            .route("/move", web::post().to(make_move)),
    );
}

async fn create_game(
    _data: web::Data<DbPool>,
    _body: web::Json<CreateGameRequest>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse> {
    let user_id = user.user_id;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "create game",
        "user_id": user_id
    })))
}

async fn join_game(
    _data: web::Data<DbPool>,
    body: web::Json<JoinGameRequest>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse> {
    let user_id = user.user_id;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "join game",
        "user_id": user_id,
        "game_id": body.game_id
    })))
}

async fn make_move(
    _data: web::Data<DbPool>,
    body: web::Json<MakeMoveRequest>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse> {
    let user_id = user.user_id;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "make move",
        "user_id": user_id,
        "game_id": body.game_id,
        "position": body.position
    })))
}

// Request types
#[derive(serde::Deserialize)]
pub struct CreateGameRequest {
    pub player_id: Uuid,
}

#[derive(serde::Deserialize)]
pub struct JoinGameRequest {
    pub game_id: Uuid,
    pub player_id: Uuid,
}

#[derive(serde::Deserialize)]
pub struct MakeMoveRequest {
    pub game_id: Uuid,
    pub player_id: Uuid,
    pub position: i32,
}
