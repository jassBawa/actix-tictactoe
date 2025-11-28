use std::sync::Arc;

use crate::manager::WsManager;
use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use uuid::Uuid;

pub async fn upgrade(
    req: HttpRequest,
    body: web::Payload,
    path_game_id: web::Path<String>,
    manager: web::Data<Arc<WsManager>>,
) -> Result<HttpResponse, Error> {
    let game_id = path_game_id.into_inner();

    let (res, mut session, mut incoming) = actix_ws::handle(&req, body)?;

    let session_id = Uuid::new_v4().to_string();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    manager.registry.add(&game_id, &session_id, tx.clone());

    rt::spawn({
        let manager = manager.clone();
        let game_id = game_id.clone();
        let session_id = session_id.clone();

        async move {
            while let Some(Ok(msg)) = incoming.next().await {
                match msg {
                    Message::Text(text) => {
                        let msg_str = text.to_string();

                        if let Err(e) = manager
                            .broadcast_except_sender(&game_id, &msg_str, &session_id)
                            .await
                        {
                            eprintln!("Broadcast error: {}", e);
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }
            manager.registry.remove(&game_id, &session_id);
        }
    });

    tokio::spawn(async move {
        while let Some(outgoing) = rx.recv().await {
            if let Err(e) = session.text(outgoing).await {
                eprintln!("Failed to send message: {}", e);
                break;
            }
        }
    });

    Ok(res)
}
