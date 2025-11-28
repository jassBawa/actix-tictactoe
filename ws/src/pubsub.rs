use crate::connection_registry::ConnectionRegistry;
use anyhow::{Context, Result};
use futures_util::StreamExt;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct MessagePayload {
    sender_id: String,
    message: String,
}

#[derive(Clone)]
pub struct PubSub {
    pub_client: Client,
    sub_client: Client,
    registry: Arc<ConnectionRegistry>,
}

impl PubSub {
    pub async fn new(redis_url: &str, registry: Arc<ConnectionRegistry>) -> Result<Self> {
        let pub_client =
            Client::open(redis_url).context("Failed to create redis publisher client")?;
        let sub_client =
            Client::open(redis_url).context("Failed to create redis subscriber client")?;

        Ok(Self {
            pub_client,
            sub_client,
            registry,
        })
    }

    pub async fn publish(
        &self,
        game_id: &str,
        message: &str,
        sender_session_id: &str,
    ) -> Result<()> {
        let channel = format!("game:{}", game_id);

        let payload = MessagePayload {
            sender_id: sender_session_id.to_string(),
            message: message.to_string(),
        };

        let json_message = serde_json::to_string(&payload)?;

        let mut conn = self
            .pub_client
            .get_multiplexed_async_connection()
            .await
            .context("Failed to get redis publisher connection")?;

        conn.publish::<_, _, i64>(channel, json_message)
            .await
            .context("Failed to publish message to pub Redis")?;

        Ok(())
    }

    pub async fn start_subscriber(&self) -> Result<()> {
        let mut pubsub = self
            .sub_client
            .get_async_pubsub()
            .await
            .context("Failed to get Redis subscriber connection")?;

        pubsub
            .psubscribe("game:*")
            .await
            .context("Failed to get Redis subscriber connection")?;

        let mut stream = pubsub.into_on_message();

        while let Some(msg) = stream.next().await {
            let json_payload = msg
                .get_payload::<String>()
                .context("Failed to get message payload")?;

            let channel = msg.get_channel_name();

            if let Some(game_id) = channel.strip_prefix("game:") {
                match serde_json::from_str::<MessagePayload>(&json_payload) {
                    Ok(payload) => {
                        self.registry.broadcast_except(
                            game_id,
                            &payload.message,
                            &payload.sender_id,
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message payload {}", e)
                    }
                }
            }
        }

        Ok(())
    }
}
