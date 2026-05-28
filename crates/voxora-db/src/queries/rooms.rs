use uuid::Uuid;
use serde::{Deserialize, Serialize};
use time::{UtcDateTime, OffsetDateTime};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub slug: String,
    pub host_id: Option<Uuid>,
    pub status: String,
    pub max_peers: i32,
    pub created_at: UtcDateTime,
    pub ended_at: Option<OffsetDateTime>,
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

async fn create_room(pool: &PgPool, new_room: NewRoom) -> Result<Room, sqlx::Error> {
    let room = sqlx::query_as!(
        Room,
        r#"
        INSERT INTO rooms (project_id, name, slug, host_id, max_peers) 
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING id, project_id, name, slug, host_id, status, max_peers, created_at, ended_at
        "#,
        new_room.project_id, 
        new_room.name, 
        new_room.slug, 
        new_room.host_id, 
        new_room.max_peers
    )
    .fetch_one(pool)
    .await;
    room
}

async fn get_room_by_id(pool: &PgPool, room_id: Uuid) -> Result<Room, sqlx::Error> {
    let room = sqlx::query_as!(Room, 
        "SELECT * FROM rooms WHERE id = $1",
        room_id
    ).fetch_optional(pool).await?;
    Ok(room)
}

async fn get_rooms_by_project_id(){}

async fn update_room(){}

async fn delete_room(){}