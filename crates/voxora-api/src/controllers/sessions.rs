use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use uuid::Uuid;
use voxora_db::{
    create_session, get_session_by_id, get_sessions_by_room_id,
    update_session, delete_session,
    NewSession, UpdateSession,
};
use crate::extractors::{
    extract_user_id, require_owner_or_admin, require_member,
    require_room_in_workspace, require_session_in_workspace,
};

/// POST /workspaces/{workspace_id}/rooms/{room_id}/sessions — Start a recording session (owner/admin only)
#[post("/workspaces/{workspace_id}/rooms/{room_id}/sessions")]
pub async fn create_session_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<NewSession>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, room_id) = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    // Chain: verify room belongs to this workspace
    require_room_in_workspace(pool.as_ref(), room_id, workspace_id).await?;

    let session = create_session(pool.as_ref(), body.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(session))
}

/// GET /workspaces/{workspace_id}/rooms/{room_id}/sessions — List all sessions in a room (any member)
#[get("/workspaces/{workspace_id}/rooms/{room_id}/sessions")]
pub async fn get_sessions_by_room_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, room_id) = path.into_inner();

    // RBAC: any member can view sessions
    require_member(pool.as_ref(), workspace_id, user_id).await?;

    // Chain: verify room belongs to this workspace
    require_room_in_workspace(pool.as_ref(), room_id, workspace_id).await?;

    let sessions = get_sessions_by_room_id(pool.as_ref(), room_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(sessions))
}

/// GET /workspaces/{workspace_id}/sessions/{session_id} — Get a single session (any member)
#[get("/workspaces/{workspace_id}/sessions/{session_id}")]
pub async fn get_session_by_id_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, session_id) = path.into_inner();

    // RBAC: any member can view a session
    require_member(pool.as_ref(), workspace_id, user_id).await?;

    // Chain: verify session belongs to this workspace
    require_session_in_workspace(pool.as_ref(), session_id, workspace_id).await?;

    let session = get_session_by_id(pool.as_ref(), session_id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Session not found"))?;

    Ok(HttpResponse::Ok().json(session))
}

/// PUT /workspaces/{workspace_id}/sessions/{session_id} — Update session status (owner/admin only)
#[put("/workspaces/{workspace_id}/sessions/{session_id}")]
pub async fn update_session_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateSession>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, session_id) = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    // Chain: verify session belongs to this workspace
    require_session_in_workspace(pool.as_ref(), session_id, workspace_id).await?;

    let session = update_session(pool.as_ref(), session_id, body.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(session))
}

/// DELETE /workspaces/{workspace_id}/sessions/{session_id} — Delete session (owner/admin only)
#[delete("/workspaces/{workspace_id}/sessions/{session_id}")]
pub async fn delete_session_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, session_id) = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    // Chain: verify session belongs to this workspace
    require_session_in_workspace(pool.as_ref(), session_id, workspace_id).await?;

    delete_session(pool.as_ref(), session_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("Session deleted successfully"))
}
