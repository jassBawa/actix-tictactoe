use dashmap::DashMap;
use tokio::sync::mpsc::UnboundedSender;
pub type SessionTx = UnboundedSender<String>;

#[derive(Clone)]
pub struct ConnectionRegistry {
    pub games: DashMap<String, DashMap<String, SessionTx>>,
}

impl ConnectionRegistry {
    pub fn new() -> Self {
        Self {
            games: DashMap::new(),
        }
    }

    pub fn add(&self, game_id: &str, session_id: &str, tx: SessionTx) {
        let game = self.games.entry(game_id.to_string()).or_default();
        game.insert(session_id.to_string(), tx);
    }

    pub fn remove(&self, game_id: &str, session_id: &str) {
        if let Some(game) = self.games.get_mut(game_id) {
            game.remove(session_id);

            if game.is_empty() {
                drop(game);
                self.games.remove(game_id);
            }
        }
    }

    pub fn broadcast_except(&self, game_id: &str, msg: &str, exclude_session_id: &str) {
        if let Some(game) = self.games.get(game_id) {
            for entry in game.value().iter() {
                let session_id = entry.key();

                if session_id != exclude_session_id {
                    let tx = entry.value();
                    let _ = tx.send(msg.to_string());
                }
            }
        }
    }
}
