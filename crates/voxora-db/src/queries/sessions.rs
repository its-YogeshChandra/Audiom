use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::{UtcDateTime, OffsetDateTime};
use sqlx::PgPool;

#[derive(Debug , Serialize , Deserialize)]
pub struct Session {
    pub id          : Uuid,
    pub room_id     : Uuid,
    pub status      : String,
    pub started_at  : OffsetDateTime,
    pub ended_at    : Option<OffsetDateTime>,
    pub duration_ms : Option<i64>,
}

#[derive(Debug , Serialize , Deserialize)]
pub struct NewSession {
    pub room_id : Uuid,
    pub status : String,
    pub started_at : OffsetDateTime,
    pub duration_ms : Option<i64>
}

#[derive(Debug , Serialize , Deserialize)]
pub struct UpdateSession {
    pub status : Option<String>
}

//db functions 

pub async fn create_session(
    pool : &PgPool,
    data : NewSession,
) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as!(
        Session,
        r#"
        INSERT INTO sessions (room_id , status , started_at , COALESCE(duration_ms , 0))
        VALUES ($1 , $2 , $3 , $4)
        RETURNING id , room_id , status , started_at , ended_at , duration_ms
        "#,
        data.room_id,
        data.status,
        data.started_at,
        data.duration_ms,
    )
    .fetch_one(pool)
    .await?;
    Ok(session)
}

pub async fn get_session_by_id(
    pool : &PgPool,
    session_id : Uuid,
) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as!(
        Session,
        r#"
        SELECT id , room_id , status , started_at , ended_at , duration_ms
        FROM sessions
        WHERE id = $1
        "#,
        session_id,
    )
    .fetch_optional(pool)
    .await?;

    match session {
        Some(session) => Ok(session),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn get_sessions_by_room_id(
    pool : &PgPool,
    room_id : Uuid,
) -> Result<Vec<Session>, sqlx::Error> {
    let sessions = sqlx::query_as!(
        Session,
        r#"
        SELECT id , room_id , status , started_at , ended_at , duration_ms
        FROM sessions
        WHERE room_id = $1
        "#,
        room_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(sessions)
}

pub async fn update_session(
    pool : &PgPool,
    session_id : Uuid,
    data : UpdateSession,
) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as!(
        Session,
        r#"
        UPDATE sessions
        SET status = COALESCE($1 , status),
            ended_at = COALESCE($2 , ended_at),
            duration_ms = COALESCE($3 , duration_ms)
        WHERE id = $4
        RETURNING id , room_id , status , started_at , ended_at , duration_ms
        "#,
        data.status,
        data.ended_at,
        data.duration_ms,
        session_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(session)
}

pub async fn delete_session(
    pool : &PgPool,
    session_id : Uuid,
) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as!(
        Session,
        r#"
        DELETE FROM sessions
        WHERE id = $1
        RETURNING id , room_id , status , started_at , ended_at , duration_ms
        "#,
        session_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(session)
}