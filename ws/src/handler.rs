use std::sync::Arc;

use crate::{
    game::messages::{WsClientMessage, WsServerMessage},
    manager::WsManager,
};
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
    user_id: Uuid,
) -> anyhow::Result<HttpResponse, Error> {
    let game_id = path_game_id.into_inner();

    let (res, mut session, mut incoming) = actix_ws::handle(&req, body)?;

    let session_id = Uuid::new_v4().to_string();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    manager.registry.add(&game_id, &session_id, tx.clone());

    rt::spawn({
        let manager = manager.clone();
        let game_id = game_id.clone();
        let session_id = session_id.clone();
        let user_id = user_id;

        async move {
            while let Some(Ok(msg)) = incoming.next().await {
                match msg {
                    Message::Text(text) => {
                        if let Err(e) = handle_message(
                            &manager,
                            &game_id,
                            &session_id,
                            user_id,
                            text.to_string(),
                        )
                        .await
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
    user_id: Uuid,
    raw: String,
) -> anyhow::Result<()> {
    match serde_json::from_str::<WsClientMessage>(&raw) {
        Ok(WsClientMessage::CreateGame) => {
            match manager
                .game_manager
                .create_game(user_id, session_id.to_string(), game_id.to_string())
                .await
            {
                Ok(game) => broadcast_state(manager, game_id, session_id, game).await?,
                Err(e) => {
                    send_error(
                        manager,
                        game_id,
                        session_id,
                        &format!("Failed to create game: {}", e),
                    )
                    .await?;
                }
            }
        }
        Ok(WsClientMessage::JoinGame) => {
            match manager
                .game_manager
                .join_game(game_id, user_id, session_id.to_string())
                .await
            {
                Ok(game) => {
                    broadcast_state(manager, game_id, session_id, game).await?;
                }
                Err(e) => {
                    send_error(
                        manager,
                        &game_id,
                        session_id,
                        &format!("Failed to join game: {}", e),
                    )
                    .await?;
                }
            }
        }
        Ok(WsClientMessage::MakeMove { position }) => {
            match manager
                .game_manager
                .make_move(game_id, user_id, position)
                .await
            {
                Ok(game) => {
                    broadcast_state(manager, game_id, session_id, game).await?;
                }
                Err(e) => {
                    send_error(
                        manager,
                        &game_id,
                        session_id,
                        &format!("Failed to make move: {}", e),
                    )
                    .await?;
                }
            }
        }

        Err(_) => {
            send_error(
                manager,
                game_id,
                session_id,
                "Invalid message format. Expected Json game message.",
            )
            .await?
        }
    }

    Ok(())
}

async fn send_error(
    manager: &Arc<WsManager>,
    game_id: &str,
    session_id: &str,
    error_message: &str,
) -> anyhow::Result<()> {
    let error_msg = WsServerMessage::Error {
        message: error_message.to_string(),
    };
    let json = serde_json::to_string(&error_msg)?;

    if let Some(game_sessions) = manager.registry.games.get(game_id) {
        if let Some(sender_tx) = game_sessions.value().get(session_id) {
            let _ = sender_tx.send(json);
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
    let payload = WsServerMessage::GameState { game };
    let json = serde_json::to_string(&payload)?;

    manager
        .broadcast_to_all(game_id, &json, sender_session_id)
        .await?;
    Ok(())
}
