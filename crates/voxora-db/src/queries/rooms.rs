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

#[derive(Debug, Serialize, Deserialize)]
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

//has to know about fetch one , fetch optional , fetch all, execute (in sqlx::query as!)


pub async fn create_room(pool: &PgPool, new_room: NewRoom) -> Result<Room, sqlx::Error> {
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

pub async fn get_room_by_id(pool: &PgPool, room_id: Uuid) -> Result<Room, sqlx::Error> {
    let room = sqlx::query_as!(Room, 
        r#"
        SELECT * FROM rooms WHERE id = $1
        "#,
        room_id
    ).fetch_optional(pool).await?;

    match room {
        Some(room) => Ok(room),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn get_rooms_by_project_id(pool : &PgPool , project_id : Uuid) -> Result<Vec<Room>, sqlx::Error>{
    
    let rooms = sqlx::query_as!(
        Room,
        r#"
        SELECT * FROM rooms WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(pool)
    .await?;
    Ok(rooms)
}

pub async fn update_room(pool : &PgPool , room_id : Uuid , update_room : UpdateRoom) -> Result<Room, sqlx::Error>{
    let room = sqlx::query_as!(
        Room,
        r#"
        UPDATE rooms 
        SET name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            host_id = COALESCE($3, host_id),
            max_peers = COALESCE($4, max_peers)
        WHERE id = $5
        RETURNING id, project_id, name, slug, host_id, status, max_peers, created_at, ended_at
        "#,
        update_room.name,
        update_room.slug,
        update_room.host_id,
        update_room.max_peers,
        room_id
    )
    .fetch_one(pool)
    .await?;
    Ok(room)
}

pub async fn update_room_status(pool : &PgPool , room_id : Uuid , status : String) -> Result<Room , sqlx::Error>{
    let room = sqlx::query_as!(
        Room,
        r#"
        UPDATE rooms 
        SET status = $1
        WHERE id = $2
        RETURNING id, project_id, name, slug, host_id, status, max_peers, created_at, ended_at
        "#,
        status,
        room_id
    )
    .fetch_one(pool)
    .await?;
    Ok(room)
}

pub async fn delete_room(pool : &PgPool , room_id : Uuid) -> Result<u64 , sqlx::Error>{
    let result = sqlx::query!(
        r#"
        DELETE FROM rooms WHERE id = $1
        "#,
        room_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}