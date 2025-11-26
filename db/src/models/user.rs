use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[sqlx(rename="password_hash")]
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
