use serde::{Deserialize, Serialize};

use crate::game::game_state::GameState;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum WsClientMessage {
    #[serde(rename = "create_game")]
    CreateGame,

    #[serde(rename = "join_game")]
    JoinGame,

    #[serde(rename = "make_move")]
    MakeMove { position: usize },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum WsServerMessage {
    #[serde(rename = "game_state")]
    GameState { game: GameState },

    #[serde(rename = "error")]
    Error { message: String },
}
