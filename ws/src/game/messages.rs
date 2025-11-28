use serde::{Deserialize, Serialize};

use crate::game::game_state::GameState;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum WsGameMessage {
    #[serde(rename = "create_game")]
    CreateGame { player_id: String },

    #[serde(rename = "join_game")]
    JoinGame { player_id: String, game_id: String },

    #[serde(rename = "make_move")]
    MakeMove { game_id: String, position: usize },

    #[serde(rename = "game_state")]
    GameState { game: GameState },

    #[serde(rename = "error")]
    Error { message: String },
}
