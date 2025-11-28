use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::room::Room;

pub struct WsManager {
    pub rooms: Arc<Mutex<HashMap<String, Arc<Mutex<Room>>>>>,
}

impl WsManager {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_room(&self, id: &str) -> Arc<Mutex<Room>> {
        let mut map = self.rooms.lock().await;

        map.entry(id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(Room::new())))
            .clone()
    }
}

pub fn start_manager() -> Arc<WsManager> {
    Arc::new(WsManager::new())
}
