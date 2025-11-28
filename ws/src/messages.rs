use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct WsClientMessage {
    pub game_id: String,
    pub user_id: String,
    pub payload: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsInternalMessage {
    pub payload: String,
}
