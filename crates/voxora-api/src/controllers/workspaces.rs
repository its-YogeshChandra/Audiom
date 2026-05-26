//workspace controller for all the workspace endpoints
use actix_web::{get, post, put, delete, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use voxora_core::verify_token;
use voxora_db::{
    create_workspace, get_workspace_by_slug, list_user_workspaces,
    update_workspace, delete_workspace, is_workspace_owner,
    get_member_role, list_workspace_members, add_member, remove_member,
    change_member_role, get_user_data, 
    NewWorkspace, UpdateWorkspace, WorkspaceRole, GetUser,
};

// ── Request bodies ──

#[derive(Deserialize)]
pub struct CreateWorkspaceBody {
    pub name: String,
    pub slug: String,
    pub plan: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateWorkspaceBody {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub plan: Option<String>,
}

#[derive(Deserialize)]
pub struct AddMemberBody {
    pub email: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct ChangeRoleBody {
    pub role: String,
}

// ── Helper: parse role string into WorkspaceRole ──

fn parse_role(role: &str) -> Result<WorkspaceRole, actix_web::Error> {
    match role {
        "owner" => Ok(WorkspaceRole::Owner),
        "admin" => Ok(WorkspaceRole::Admin),
        "editor" => Ok(WorkspaceRole::Editor),
        "viewer" => Ok(WorkspaceRole::Viewer),
        "guest" => Ok(WorkspaceRole::Guest),
        _ => Err(actix_web::error::ErrorBadRequest("Invalid role. Must be: owner, admin, editor, viewer, or guest")),
    }
}

// ── Helper: extract Bearer token from Authorization header ──

fn extract_bearer_token(req: &HttpRequest) -> Result<String, actix_web::Error> {
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

fn extract_user_id(req: &HttpRequest) -> Result<Uuid, actix_web::Error> {
    let token = extract_bearer_token(req)?;
    let claims = verify_token(&token)
        .map_err(|e| actix_web::error::ErrorUnauthorized(format!("Invalid token: {}", e)))?;

    claims.id.ok_or_else(|| actix_web::error::ErrorUnauthorized("Token missing user id"))
}

// ── Helper: check if user is owner or admin ──

async fn require_owner_or_admin(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), actix_web::Error> {
    let role = get_member_role(pool, workspace_id, user_id).await
        .map_err(|_| actix_web::error::ErrorForbidden("You are not a member of this workspace"))?;

    match role {
        WorkspaceRole::Owner | WorkspaceRole::Admin => Ok(()),
        _ => Err(actix_web::error::ErrorForbidden("Only owners and admins can perform this action")),
    }
}

// ── Helper: check if user is owner only ──

async fn require_owner(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), actix_web::Error> {
    if !is_workspace_owner(pool, workspace_id, user_id).await {
        return Err(actix_web::error::ErrorForbidden("Only the workspace owner can perform this action"));
    }
    Ok(())
}

// ── Helper: check if user is a member ──

async fn require_member(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), actix_web::Error> {
    get_member_role(pool, workspace_id, user_id).await
        .map_err(|_| actix_web::error::ErrorForbidden("You are not a member of this workspace"))?;
    Ok(())
}

// ── Endpoints ──

/// POST /workspaces — Any authenticated user can create a workspace
#[post("/workspaces")]
pub async fn create_workspace_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    body: web::Json<CreateWorkspaceBody>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();

    let new_ws = NewWorkspace {
        name: body.name.clone(),
        slug: body.slug.clone(),
        owner_id: user_id,
        plan: body.plan.clone().unwrap_or_else(|| "free".to_string()),
    };

    // Create the workspace
    let workspace = create_workspace(pool, new_ws).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // Auto-add the creator as owner in workspace_members
    add_member(pool, workspace.id, user_id, WorkspaceRole::Owner).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(workspace))
}

/// GET /workspaces — List all workspaces the authenticated user belongs to
#[get("/workspaces")]
pub async fn get_workspaces_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();

    let workspaces = list_user_workspaces(pool, user_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(workspaces))
}

/// GET /workspaces/{slug} — Get workspace by slug (members only)
#[get("/workspaces/{slug}")]
pub async fn get_workspace_by_slug_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();
    let slug = path.into_inner();

    let workspace = get_workspace_by_slug(pool, &slug).await
        .map_err(|_| actix_web::error::ErrorNotFound("Workspace not found"))?;

    // RBAC: must be a member to view
    require_member(pool, workspace.id, user_id).await?;

    Ok(HttpResponse::Ok().json(workspace))
}

/// PUT /workspaces/{workspace_id} — Update workspace (owner/admin only)
#[put("/workspaces/{workspace_id}")]
pub async fn update_workspace_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateWorkspaceBody>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();
    let workspace_id = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool, workspace_id, user_id).await?;

    let update = UpdateWorkspace {
        name: body.name.clone(),
        slug: body.slug.clone(),
        owner_id: None, // owner_id cannot be changed via this endpoint
        plan: body.plan.clone(),
    };

    update_workspace(pool, workspace_id, update).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("Workspace updated successfully"))
}

/// DELETE /workspaces/{workspace_id} — Delete workspace (owner only)
#[delete("/workspaces/{workspace_id}")]
pub async fn delete_workspace_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();
    let workspace_id = path.into_inner();

    // RBAC: owner only
    require_owner(pool, workspace_id, user_id).await?;

    delete_workspace(pool, workspace_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("Workspace deleted successfully"))
}

/// GET /workspaces/{workspace_id}/members — List members (members only)
#[get("/workspaces/{workspace_id}/members")]
pub async fn get_workspace_members_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();
    let workspace_id = path.into_inner();

    // RBAC: must be a member to see the member list
    require_member(pool, workspace_id, user_id).await?;

    let members = list_workspace_members(pool, workspace_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(members))
}

/// POST /workspaces/{workspace_id}/members — Add member by email (owner/admin only)
#[post("/workspaces/{workspace_id}/members")]
pub async fn add_workspace_member_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AddMemberBody>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();
    let workspace_id = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool, workspace_id, user_id).await?;

    let role = parse_role(&body.role)?;

    // Cannot add someone as owner — there's only one owner
    if matches!(role, WorkspaceRole::Owner) {
        return Err(actix_web::error::ErrorBadRequest("Cannot add a member as owner. Transfer ownership instead."));
    }

    // Look up the user by email
    let target_user = get_user_data(pool, GetUser::Email(body.email.clone())).await
        .map_err(|_| actix_web::error::ErrorNotFound("User with this email not found. They must sign up first."))?;

    let member = add_member(pool, workspace_id, target_user.id, role).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(member))
}

/// DELETE /workspaces/{workspace_id}/members/{target_user_id} — Remove member (owner/admin only)
#[delete("/workspaces/{workspace_id}/members/{target_user_id}")]
pub async fn remove_workspace_member_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();
    let (workspace_id, target_user_id) = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool, workspace_id, user_id).await?;

    // Cannot remove the owner
    if is_workspace_owner(pool, workspace_id, target_user_id).await {
        return Err(actix_web::error::ErrorBadRequest("Cannot remove the workspace owner"));
    }

    // An admin cannot remove another admin — only owner can
    let target_role = get_member_role(pool, workspace_id, target_user_id).await
        .map_err(|_| actix_web::error::ErrorNotFound("User is not a member of this workspace"))?;
    
    if matches!(target_role, WorkspaceRole::Admin) {
        require_owner(pool, workspace_id, user_id).await?;
    }

    remove_member(pool, workspace_id, target_user_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("Member removed successfully"))
}

/// PUT /workspaces/{workspace_id}/members/{target_user_id}/role — Change role (owner/admin only)
#[put("/workspaces/{workspace_id}/members/{target_user_id}/role")]
pub async fn change_workspace_member_role_endpoint(
    req: HttpRequest,
    pgpool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ChangeRoleBody>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let pool = pgpool.as_ref();
    let (workspace_id, target_user_id) = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool, workspace_id, user_id).await?;

    let new_role = parse_role(&body.role)?;

    // Cannot change anyone to owner
    if matches!(new_role, WorkspaceRole::Owner) {
        return Err(actix_web::error::ErrorBadRequest("Cannot assign owner role. Transfer ownership instead."));
    }

    // Cannot change the owner's role
    if is_workspace_owner(pool, workspace_id, target_user_id).await {
        return Err(actix_web::error::ErrorBadRequest("Cannot change the owner's role"));
    }

    // An admin cannot change another admin's role — only owner can
    let target_role = get_member_role(pool, workspace_id, target_user_id).await
        .map_err(|_| actix_web::error::ErrorNotFound("User is not a member of this workspace"))?;

    if matches!(target_role, WorkspaceRole::Admin) {
        require_owner(pool, workspace_id, user_id).await?;
    }

    change_member_role(pool, workspace_id, target_user_id, new_role).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("Member role updated successfully"))
}