use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum GameStatus {
    Waiting,
    InProgress,
    Finished,
    Abondoned,
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
    pub player1_id: String,
    pub player1_session: String,
    pub player2_id: Option<String>,
    pub player2_session: Option<String>,
    pub current_turn: Option<String>,
    pub status: GameStatus,
    pub board: Board,
    pub winner: Option<String>,
    pub created_at: i64,
}

impl GameState {
    pub fn new(game_id: String, player1_id: String, player1_session: String) -> Self {
        Self {
            id: game_id,
            player1_id,
            player1_session: player1_session.clone(),
            player2_id: None,
            player2_session: None,
            current_turn: Some(player1_session.clone()),
            status: GameStatus::Waiting,
            board: [[None; 3]; 3],
            winner: None,
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn get_player_symbol(&self, session_id: &str) -> Option<Player> {
        if self.player1_session == session_id {
            Some(Player::X)
        } else if self.player2_session.as_ref() == Some(&session_id.to_string()) {
            Some(Player::O)
        } else {
            None
        }
    }

    pub fn is_player_turn(&self, session_id: &str) -> bool {
        self.current_turn.as_ref() == Some(&session_id.to_string())
    }
}
