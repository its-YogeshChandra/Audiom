use uuid::Uuid;
use sqlx::PgPool;
use serde::Serialize;

//database apis for the workspaces table
#[derive(Debug, Serialize)]
pub struct Workspace {
    pub id         : Uuid,
    pub name       : String,
    pub slug       : String,
    pub owner_id   : Uuid,
    pub plan       : String,
    pub created_at : time::OffsetDateTime
}

#[derive(Debug, Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum WorkspaceRole {
    Owner,
    Admin,
    Editor,
    Viewer,
    Guest
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WorkspaceMembers {
    pub workspace_id : Uuid,
    pub user_id      : Uuid,
    pub role         : WorkspaceRole,
    pub joined_at    : time::OffsetDateTime
}

pub struct NewWorkspace {
    pub name       : String,
    pub slug       : String,
    pub owner_id   : Uuid,
    pub plan       : String,
}

pub struct UpdateWorkspace {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub owner_id: Option<Uuid>,
    pub plan: Option<String>,
}

// ── Workspace CRUD ──

pub async fn create_workspace(pool: &PgPool, new_workspace: NewWorkspace) -> Result<Workspace, sqlx::Error> {
    let workspace = sqlx::query_as!(Workspace,
        "INSERT INTO workspaces (name, slug, owner_id, plan) VALUES ($1, $2, $3, $4) RETURNING *",
        new_workspace.name,
        new_workspace.slug,
        new_workspace.owner_id,
        new_workspace.plan
    )
    .fetch_one(pool)
    .await?;

    Ok(workspace)
}

pub async fn get_workspace_by_id(pool: &PgPool, workspace_id: Uuid) -> Result<Workspace, sqlx::Error> {
    let workspace = sqlx::query_as!(Workspace, "SELECT * FROM workspaces WHERE id = $1", workspace_id)
        .fetch_optional(pool)
        .await?;

    match workspace {
        Some(value) => Ok(value),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn get_workspace_by_slug(pool: &PgPool, slug: &str) -> Result<Workspace, sqlx::Error> {
    let workspace = sqlx::query_as!(Workspace, "SELECT * FROM workspaces WHERE slug = $1", slug)
        .fetch_optional(pool)
        .await?;

    match workspace {
        Some(value) => Ok(value),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn list_user_workspaces(pool: &PgPool, user_id: Uuid) -> Result<Vec<Workspace>, sqlx::Error> {
    let workspaces = sqlx::query_as!(Workspace,
        r#"SELECT w.* FROM workspaces w
           INNER JOIN workspace_members wm ON w.id = wm.workspace_id
           WHERE wm.user_id = $1
           ORDER BY w.created_at DESC"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(workspaces)
}

pub async fn update_workspace(pool: &PgPool, workspace_id: Uuid, update: UpdateWorkspace) -> Result<(), sqlx::Error> {
    let query = r#"
        UPDATE workspaces
        SET
            name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            owner_id = COALESCE($3, owner_id),
            plan = COALESCE($4, plan)
        WHERE id = $5
    "#;

    sqlx::query(query)
        .bind(update.name)
        .bind(update.slug)
        .bind(update.owner_id)
        .bind(update.plan)
        .bind(workspace_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_workspace(pool: &PgPool, workspace_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM workspaces WHERE id = $1")
        .bind(workspace_id)
        .execute(pool)
        .await?;
    Ok(())
}

// ── Ownership check ──

pub async fn is_workspace_owner(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> bool {
    let workspace = get_workspace_by_id(pool, workspace_id).await;
    match workspace {
        Ok(value) => value.owner_id == user_id,
        Err(_) => false,
    }
}

// ── Membership ──

pub async fn get_member_role(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<WorkspaceRole, sqlx::Error> {
    let member = sqlx::query_as!(WorkspaceMembers,
        r#"SELECT workspace_id, user_id, role as "role: WorkspaceRole", joined_at
           FROM workspace_members
           WHERE workspace_id = $1 AND user_id = $2"#,
        workspace_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match member {
        Some(m) => Ok(m.role),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn list_workspace_members(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<WorkspaceMembers>, sqlx::Error> {
    let members = sqlx::query_as!(WorkspaceMembers,
        r#"SELECT workspace_id, user_id, role as "role: WorkspaceRole", joined_at
           FROM workspace_members
           WHERE workspace_id = $1
           ORDER BY joined_at ASC"#,
        workspace_id
    )
    .fetch_all(pool)
    .await?;

    Ok(members)
}

pub async fn add_member(pool: &PgPool, workspace_id: Uuid, user_id: Uuid, role: WorkspaceRole) -> Result<WorkspaceMembers, sqlx::Error> {
    let member = sqlx::query_as!(WorkspaceMembers,
        r#"INSERT INTO workspace_members (workspace_id, user_id, role)
           VALUES ($1, $2, $3)
           RETURNING workspace_id, user_id, role as "role: WorkspaceRole", joined_at"#,
        workspace_id,
        user_id,
        role as WorkspaceRole
    )
    .fetch_one(pool)
    .await?;

    Ok(member)
}

pub async fn remove_member(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM workspace_members WHERE workspace_id = $1 AND user_id = $2")
        .bind(workspace_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn change_member_role(pool: &PgPool, workspace_id: Uuid, user_id: Uuid, role: WorkspaceRole) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE workspace_members SET role = $1 WHERE workspace_id = $2 AND user_id = $3")
        .bind(role as WorkspaceRole)
        .bind(workspace_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}