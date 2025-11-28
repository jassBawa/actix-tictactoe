use anyhow::Result;
use std::sync::Arc;

use crate::connection_registry::ConnectionRegistry;
use crate::pubsub::PubSub;

pub struct WsManager {
    pub registry: Arc<ConnectionRegistry>,
    pub pubsub: Arc<PubSub>,
}

impl WsManager {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let registry = Arc::new(ConnectionRegistry::new());
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
}

pub async fn start_manager(redis_url: &str) -> anyhow::Result<Arc<WsManager>> {
    Ok(Arc::new(WsManager::new(redis_url).await?))
}
