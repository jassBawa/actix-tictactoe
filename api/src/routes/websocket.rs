use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Result, web};
use ws::{handler, manager::WsManager};

pub fn config(cfg: &mut web::ServiceConfig, ws_manager: web::Data<Arc<WsManager>>) {
    cfg.service(
        web::scope("/ws")
            .app_data(ws_manager)
            .route("/{game_id}", web::get().to(websocket_handler)),
    );
}

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    game_id: web::Path<String>,
    manager: web::Data<Arc<WsManager>>,
) -> Result<HttpResponse> {
    handler::upgrade(req, stream, game_id, manager)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("WebSocket error: {}", e)))
}
