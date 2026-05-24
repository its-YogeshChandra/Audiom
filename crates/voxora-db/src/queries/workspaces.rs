use uuid::Uuid;
use sqlx::PgPool;

//database apis for the workspaces table
pub struct Workspace {
    pub id         : Uuid,
    pub name       : String,
    pub slug       : String,
    pub owner_id   : Uuid,
    pub plan       : String,
    pub created_at : time::OffsetDateTime

}

pub struct WorkspaceMembers {
    pub workspace_id : Uuid, 
    pub user_id      : Uuid, 
    pub role         : String,
    pub joined_at    : time::OffsetDateTime
}

//query function for the workspace 
pub struct NewWorkspace {
    pub name       : String,
    pub slug       : String,
    pub owner_id   : Uuid,
    pub plan       : String,
}

//create workspace 
async fn create_workspace (pool: PgPool, new_workspace : NewWorkspace) -> Result<Workspace, sqlx::Error> {
    let workspace = sqlx::query_as!(Workspace, 
        "INSERT INTO workspaces (name, slug, owner_id, plan) VALUES ($1, $2, $3, $4) RETURNING *",
     new_workspace.name,
     new_workspace.slug,
     new_workspace.owner_id,
     new_workspace.plan
    )
    .fetch_optional(&pool)
    .await?;

    match workspace {
        Some(value)=> Ok(value),
        //the error should be handled (future me problem)
        None => Err(sqlx::Error::RowNotFound),
    }
}


