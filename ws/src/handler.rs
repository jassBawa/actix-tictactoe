use std::sync::Arc;

use crate::{game::messages::WsGameMessage, manager::WsManager};
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
                        if let Err(e) =
                            handle_message(&manager, &game_id, &session_id, text.to_string()).await
                        {
                            eprintln!("Message handling error: {}", e);
                        }
                    }
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

async fn handle_message(
    manager: &Arc<WsManager>,
    game_id: &str,
    session_id: &str,
    raw: String,
) -> anyhow::Result<()> {
    match serde_json::from_str::<WsGameMessage>(&raw) {
        Ok(WsGameMessage::CreateGame { player_id }) => {
            let game = manager
                .game_manager
                .create_game(player_id, session_id.to_string(), game_id.to_string())
                .await?;

            broadcast_state(manager, game_id, session_id, game).await?;
        }
        Ok(WsGameMessage::JoinGame { player_id, game_id }) => {
            let game = manager
                .game_manager
                .join_game(&game_id, player_id, session_id.to_string())
                .await?;

            broadcast_state(manager, &game_id, session_id, game).await?;
        }
        Ok(WsGameMessage::MakeMove { game_id, position }) => {
            let game = manager
                .game_manager
                .make_move(&game_id, session_id, position)
                .await?;

            broadcast_state(manager, &game_id, session_id, game).await?;
        }
        Ok(WsGameMessage::GameState { .. }) => {
            // Messages of this type should come from the server only; ignore
        }
        Ok(WsGameMessage::Error { .. }) => {
            // Clients shouldn't send this; ignore
        }
        Err(_) => {
            // Fallback: treat as raw text broadcast (optional)
            manager
                .broadcast_except_sender(game_id, &raw, session_id)
                .await?;
        }
    }

    Ok(())
}

async fn broadcast_state(
    manager: &Arc<WsManager>,
    game_id: &str,
    sender_session_id: &str,
    game: crate::game::game_state::GameState,
) -> anyhow::Result<()> {
    let payload = WsGameMessage::GameState { game };
    let json = serde_json::to_string(&payload)?;
    manager
        .broadcast_except_sender(game_id, &json, sender_session_id)
        .await?;
    Ok(())
}
