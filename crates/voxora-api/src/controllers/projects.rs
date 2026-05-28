//api for the projects
use actix_web::{get, post, put, delete, web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use voxora_db::{
    create_project, get_project_by_id, get_project_by_workspace_id,
    update_project_by_id, delete_project_by_id,
    NewProject, UpdateProject,
};
use crate::extractors::{
    extract_user_id, require_owner_or_admin, require_member,
};

/// POST /workspaces/{workspace_id}/projects — Create project (owner/admin only)
#[post("/workspaces/{workspace_id}/projects")]
pub async fn create_project_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<NewProject>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let workspace_id = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    let project = create_project(pool.as_ref(), workspace_id, body.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(project))
}

/// GET /workspaces/{workspace_id}/projects — List all projects in workspace (any member)
#[get("/workspaces/{workspace_id}/projects")]
pub async fn get_projects_by_workspace_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let workspace_id = path.into_inner();

    // RBAC: any member can list projects
    require_member(pool.as_ref(), workspace_id, user_id).await?;

    let projects = get_project_by_workspace_id(pool.as_ref(), workspace_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(projects))
}

/// GET /workspaces/{workspace_id}/projects/{project_id} — Get single project (any member)
#[get("/workspaces/{workspace_id}/projects/{project_id}")]
pub async fn get_project_by_id_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, project_id) = path.into_inner();

    // RBAC: any member can view a project
    require_member(pool.as_ref(), workspace_id, user_id).await?;

    let project = get_project_by_id(pool.as_ref(), project_id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Project not found"))?;

    // Verify the project actually belongs to this workspace
    if project.workspace_id != workspace_id {
        return Err(actix_web::error::ErrorNotFound("Project not found in this workspace"));
    }

    Ok(HttpResponse::Ok().json(project))
}

/// PUT /workspaces/{workspace_id}/projects/{project_id} — Update project (owner/admin only)
#[put("/workspaces/{workspace_id}/projects/{project_id}")]
pub async fn update_project_by_id_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateProject>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, project_id) = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    // Verify the project belongs to this workspace
    let existing = get_project_by_id(pool.as_ref(), project_id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Project not found"))?;

    if existing.workspace_id != workspace_id {
        return Err(actix_web::error::ErrorNotFound("Project not found in this workspace"));
    }

    let project = update_project_by_id(pool.as_ref(), project_id, body.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(project))
}

/// DELETE /workspaces/{workspace_id}/projects/{project_id} — Delete project (owner/admin only)
#[delete("/workspaces/{workspace_id}/projects/{project_id}")]
pub async fn delete_project_by_id_controller(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, project_id) = path.into_inner();

    // RBAC: owner or admin only
    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    // Verify the project belongs to this workspace
    let existing = get_project_by_id(pool.as_ref(), project_id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Project not found"))?;

    if existing.workspace_id != workspace_id {
        return Err(actix_web::error::ErrorNotFound("Project not found in this workspace"));
    }

    delete_project_by_id(pool.as_ref(), project_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("Project deleted successfully"))
}
