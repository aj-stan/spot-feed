use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_guest: bool,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn from_row(row: &Row) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            is_guest: row.get("is_guest"),
            created_at: row.get("created_at"),
        }
    }
}
