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
    pub duration_ms : Option<i32>
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
        INSERT INTO sessions (room_id, status, started_at, duration_ms)
        VALUES ($1, $2, $3, COALESCE($4, 0))
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
        SET status = COALESCE($1 , status)
        WHERE id = $2
        RETURNING id , room_id , status , started_at , ended_at , duration_ms
        "#,
        data.status,
        session_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(session)
}

pub async fn delete_session(
    pool : &PgPool,
    session_id : Uuid,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM sessions WHERE id = $1
        "#,
        session_id
    )
    .execute(pool)
    .await?;
   
    Ok(result.rows_affected())
}

// ── Chain verification: session belongs to workspace ──

pub async fn verify_session_in_workspace(pool: &PgPool, session_id: Uuid, workspace_id: Uuid) -> bool {
    let result = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM sessions s
            JOIN rooms r ON s.room_id = r.id
            JOIN projects p ON r.project_id = p.id
            WHERE s.id = $1 AND p.workspace_id = $2
        ) as "exists!"
        "#,
        session_id,
        workspace_id
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(exists) => exists,
        Err(_) => false,
    }
}