use sqlx::PgPool;
use uuid::Uuid;
use voxora_core::verify_token;
use voxora_db::{
    get_member_role, is_workspace_owner,
    WorkspaceRole,
};
use actix_web::{HttpRequest};


pub fn extract_bearer_token(req: &HttpRequest) -> Result<String, actix_web::Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?
        .to_str()
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(actix_web::error::ErrorUnauthorized("Authorization header must start with 'Bearer '"));
    }

    Ok(auth_header["Bearer ".len()..].to_string())
}

// ── Helper: extract user_id from JWT token in Authorization header ──

pub fn extract_user_id(req: &HttpRequest) -> Result<Uuid, actix_web::Error> {
    let token = extract_bearer_token(req)?;
    let claims = verify_token(&token)
        .map_err(|e| actix_web::error::ErrorUnauthorized(format!("Invalid token: {}", e)))?;

    claims.id.ok_or_else(|| actix_web::error::ErrorUnauthorized("Token missing user id"))
}

// ── Helper: check if user is owner or admin ──

pub async fn require_owner_or_admin(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), actix_web::Error> {
    let role = get_member_role(pool, workspace_id, user_id).await
        .map_err(|_| actix_web::error::ErrorForbidden("You are not a member of this workspace"))?;

    match role {
        WorkspaceRole::Owner | WorkspaceRole::Admin => Ok(()),
        _ => Err(actix_web::error::ErrorForbidden("Only owners and admins can perform this action")),
    }
}

// ── Helper: check if user is owner only ──

pub async fn require_owner(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), actix_web::Error> {
    if !is_workspace_owner(pool, workspace_id, user_id).await {
        return Err(actix_web::error::ErrorForbidden("Only the workspace owner can perform this action"));
    }
    Ok(())
}

// ── Helper: check if user is a member ──
pub async fn require_member(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), actix_web::Error> {
    get_member_role(pool, workspace_id, user_id).await
        .map_err(|_| actix_web::error::ErrorForbidden("You are not a member of this workspace"))?;
    Ok(())
}