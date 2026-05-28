use uuid::Uuid;
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub slug: String,
    pub host_id: Uuid,
    pub status: String,
    pub max_peers: i32,
    pub created_at: UtcDateTime,
    pub ended_at: UtcDateTime,
}

pub struct NewRoom {
    pub project_id: Uuid,
    pub name: String,
    pub slug: String,
    pub host_id: Uuid,
    pub max_peers: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoom{
    pub name: Option<String>,
    pub slug: Option<String>,
    pub host_id: Option<Uuid>,
    pub max_peers: Option<i32>,
}

