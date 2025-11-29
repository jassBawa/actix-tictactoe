use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum GameStatus {
    Waiting,
    InProgress,
    Finished,
    Abandoned,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

pub type Board = [[Option<Player>; 3]; 3];

// |Player Player Player|
// |Player Player Player|
// |Player Player Player|
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub id: String,
    pub player1_id: Uuid,
    pub player2_id: Option<Uuid>,

    pub player1_session: String,
    pub player2_session: Option<String>,

    pub current_turn: Option<Uuid>,

    pub status: GameStatus,
    pub board: Board,
    pub winner: Option<Uuid>,
    pub created_at: i64,
}

impl GameState {
    pub fn new(game_id: String, player1_id: Uuid, player1_session: String) -> Self {
        Self {
            id: game_id,
            player1_id,
            player1_session,
            player2_id: None,
            player2_session: None,
            current_turn: Some(player1_id),
            status: GameStatus::Waiting,
            board: [[None; 3]; 3],
            winner: None,
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn get_player_symbol(&self, user_id: Uuid) -> Option<Player> {
        if self.player1_id == user_id {
            Some(Player::X)
        } else if self.player2_id == Some(user_id) {
            Some(Player::O)
        } else {
            None
        }
    }

    pub fn is_player_turn(&self, user_id: Uuid) -> bool {
        self.current_turn == Some(user_id)
    }
}
