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

    let room = manager.get_or_create_room(&game_id).await;

    let (res, mut session, mut incoming) = actix_ws::handle(&req, body)?;

    let session_id = Uuid::new_v4().to_string();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    {
        let mut room = room.lock().await;
        room.join(session_id.clone(), tx);
    }

    rt::spawn({
        let room = Arc::clone(&room);
        let session_id = session_id.clone();

        async move {
            while let Some(Ok(msg)) = incoming.next().await {
                match msg {
                    Message::Text(text) => {
                        let room = room.lock().await;
                        room.broadcast(text.to_string());
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }

            // Remove client on disconnect
            let mut room = room.lock().await;
            room.leave(&session_id);
        }
    });

    tokio::spawn(async move {
        while let Some(outgoing) = rx.recv().await {
            let _ = session.text(outgoing).await;
        }
    });

    Ok(res)
}
