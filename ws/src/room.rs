use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};

pub struct Room {
    pub clients: HashMap<String, mpsc::UnboundedSender<String>>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn join(&mut self, id: String, tx: mpsc::UnboundedSender<String>) {
        self.clients.insert(id, tx);
    }

    pub fn leave(&mut self, id: &str) {
        self.clients.remove(id);
    }

    pub fn broadcast(&self, msg: String) {
        for tx in self.clients.values() {
            let _ = tx.send(msg.clone());
        }
    }

    pub fn broadcast_except(&self, msg: String, exclude_id: &str) {
        for (id, tx) in &self.clients {
            if id != exclude_id {
                let _ = tx.send(msg.clone());
            }
        }
    }
}
