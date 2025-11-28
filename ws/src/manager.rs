use anyhow::{Context, Result};
use redis::Client;
use std::sync::Arc;

use crate::connection_registry::ConnectionRegistry;
use crate::game::game_manager::GameManager;
use crate::pubsub::PubSub;

pub struct WsManager {
    pub registry: Arc<ConnectionRegistry>,
    pub pubsub: Arc<PubSub>,
    pub game_manager: Arc<GameManager>,
}

impl WsManager {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let registry = Arc::new(ConnectionRegistry::new());

        let redis_client = Arc::new(
            Client::open(redis_url).context("Failed to create redis client for gamemanager")?,
        );

        let game_manager = Arc::new(GameManager::new(Arc::clone(&redis_client)));
        let pubsub = PubSub::new(redis_url, Arc::clone(&registry)).await?;

        let pubsub_for_subscriber = pubsub.clone();

        tokio::spawn(async move {
            if let Err(e) = pubsub_for_subscriber.start_subscriber().await {
                eprintln!("Redis subscriber error: {}", e);
            }
        });

        Ok(Self {
            registry,
            pubsub: Arc::new(pubsub),
            game_manager,
        })
    }

    pub async fn broadcast_except_sender(
        &self,
        game_id: &str,
        message: &str,
        sender_session_id: &str,
    ) -> anyhow::Result<()> {
        self.pubsub
            .publish(game_id, message, sender_session_id)
            .await?;
        Ok(())
    }
    pub async fn broadcast_to_all(
        &self,
        game_id: &str,
        message: &str,
        sender_session_id: &str,
    ) -> Result<()> {
        if let Some(game_sessions) = self.registry.games.get(game_id) {
            if let Some(sender_tx) = game_sessions.value().get(sender_session_id) {
                let _ = sender_tx.send(message.to_string());
            }
        }

        self.pubsub
            .publish(game_id, message, sender_session_id)
            .await?;

        Ok(())
    }
}

pub async fn start_manager(redis_url: &str) -> anyhow::Result<Arc<WsManager>> {
    Ok(Arc::new(WsManager::new(redis_url).await?))
}
