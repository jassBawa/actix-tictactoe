use anyhow::{Context, Result};
use redis::{AsyncTypedCommands, Client};
use std::sync::Arc;

use crate::game::{
    game_logic::GameEngine,
    game_state::{GameState, GameStatus, Player},
};

#[derive(Clone)]
pub struct GameManager {
    redis_client: Arc<Client>,
}

impl GameManager {
    pub fn new(redis_client: Arc<Client>) -> Self {
        Self { redis_client }
    }

    pub async fn create_game(
        &self,
        player1_id: String,
        player1_session: String,
        game_id: String,
    ) -> Result<GameState> {
        let game = GameState::new(game_id.clone(), player1_id, player1_session);

        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let game_json = serde_json::to_string(&game)?;
        let key = format!("game:{}", game_id);

        conn.set_ex(key, game_json, 3600)
            .await
            .context("Failed to set key in redis")?;

        Ok(game)
    }

    pub async fn get_game(&self, game_id: &str) -> Result<Option<GameState>> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!("game:{}", game_id);
        let game_json: Option<String> = conn.get(key).await?;

        match game_json {
            Some(json) => {
                let json = serde_json::from_str(&json)?;
                Ok(json)
            }
            None => Ok(None),
        }
    }

    pub async fn join_game(
        &self,
        game_id: &str,
        player2_id: String,
        player2_session: String,
    ) -> Result<GameState> {
        let mut game = self.get_game(game_id).await?.context("Game not found")?;

        if game.status != GameStatus::Waiting {
            anyhow::bail!("Game is not waiting for players");
        }

        game.player2_id = Some(player2_id);
        game.player2_session = Some(player2_session);
        game.status = GameStatus::InProgress;

        self.save_game(&game).await?;
        Ok(game)
    }

    pub async fn make_move(
        &self,
        game_id: &str,
        session_id: &str,
        position: usize,
    ) -> Result<GameState> {
        let mut game = self.get_game(game_id).await?.context("Game not found")?;

        if game.status != GameStatus::InProgress {
            anyhow::bail!("Game is not in progress");
        }

        if !game.is_player_turn(session_id) {
            anyhow::bail!("Not your turn");
        }

        let player = game
            .get_player_symbol(session_id)
            .context("Invalid player")?;

        GameEngine::make_move(&mut game.board, position, player)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let winner = GameEngine::check_winner(&game.board);

        match winner {
            Some(Player::X) => {
                game.status = GameStatus::Finished;
                game.winner = Some(game.player1_session.clone());
            }
            Some(Player::O) => {
                game.status = GameStatus::Finished;
                game.winner = game.player2_session.clone();
            }
            None if GameEngine::is_board_full(&game.board) => {
                game.status = GameStatus::Finished;
                game.winner = None;
            }

            None => {
                let next = if session_id == game.player1_session {
                    game.player2_session.clone().unwrap()
                } else {
                    game.player1_session.clone()
                };
                game.current_turn = Some(next);
            }
        }

        self.save_game(&game).await?;
        Ok(game)
    }

    async fn save_game(&self, game: &GameState) -> Result<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!("game:{}", game.id);
        let game_json = serde_json::to_string(game)?;

        conn.set_ex(key, game_json, 3600).await?;

        Ok(())
    }

    pub async fn delete_game(&self, game_id: &str) -> Result<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!("game:{}", game_id);

        conn.del(key).await?;

        Ok(())
    }
}
