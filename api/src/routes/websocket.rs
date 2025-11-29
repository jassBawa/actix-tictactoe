use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Result, web};
use actix_web_lab::middleware::from_fn;
use ws::{handler, manager::WsManager};

use crate::middleware::{extract_user_from_request, jwt_auth_fn};

pub fn config(cfg: &mut web::ServiceConfig, ws_manager: web::Data<Arc<WsManager>>) {
    cfg.service(
        web::scope("/ws")
            .app_data(ws_manager)
            .wrap(from_fn(jwt_auth_fn))
            .route("/{game_id}", web::get().to(websocket_handler)),
    );
}

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    game_id: web::Path<String>,
    manager: web::Data<Arc<WsManager>>,
) -> Result<HttpResponse> {
    let user_id = extract_user_from_request(&req).map_err(|e| {
        actix_web::error::ErrorUnauthorized(format!("Authentication failed: {}", e))
    })?;

    handler::upgrade(req, stream, game_id, manager, user_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("WebSocket error: {}", e)))
}
