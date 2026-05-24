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

pub enum WorkspaceRole {
    Owner(String),
    Admin(String),
    Editor(String), 
    Viewer(String), 
    Guest(String)
}


pub struct WorkspaceMembers {
    pub workspace_id : Uuid, 
    pub user_id      : Uuid, 
    pub role         : WorkspaceRole, 
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

async fn get_workspace_owner(pool: PgPool, workspace_id: Uuid) -> Result<Workspace, sqlx::Error> {
    let workspace = sqlx::query_as!(Workspace, "SELECT * FROM workspaces WHERE id = $1", workspace_id)
        .fetch_optional(&pool)
        .await?;

    match workspace {
        Some(value) => Ok(value),
        None => Err(sqlx::Error::RowNotFound),
    }
}

async fn get_workspace_member(pool: PgPool, workspace_id: Uuid) -> Result<Workspace, sqlx::Error> {
    let workspace = sqlx::query_as!(Workspace, "SELECT * FROM workspaces WHERE id = $1", workspace_id)
        .fetch_optional(&pool)
        .await?;

    match workspace {
        //filter the values in here 
        Some(value) => Ok(value),
        None => Err(sqlx::Error::RowNotFound),
    }
}


async fn is_workspace_owner(pool: PgPool, workspace_id :Uuid, user_id : Uuid,  ) ->  bool {
    //check first the user is the owner 
    let workspace = get_workspace_owner(pool, workspace_id).await;
    
    match workspace {
        Ok(value)=> {
            if value.owner_id != user_id {
                return false
            }
            return true
        }
        Err(_) => {
            //do error handling future me problem 
            return false
        }
    }

}


pub struct UpdateWorkspace{
 pub name: Option<String>,
 pub slug: Option<String>,
 pub owner_id: Option<Uuid>,
 pub plan: Option<String>,    
}

//there is two distinction first either upate one value at a time 
//either multiple values can be updated at a time 

async fn update_workspace (pool: PgPool, workspace_id:Uuid, update_workspace : UpdateWorkspace) -> Result<(), sqlx::Error> {
  let query = r#"
        UPDATE workspaces
        SET 
            name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            owner_id = COALESCE($3, owner_id),
            plan = COALESCE($4, plan),
            updated_at = NOW()
        WHERE id = $5
    "#;

    sqlx::query(query)
        // SQLx binds `Option<T>` directly as `T` or `NULL`
        .bind(update_workspace.name) 
        .bind(update_workspace.slug)
        .bind(update_workspace.owner_id)
        .bind(update_workspace.plan)
        .bind(workspace_id)
        .execute(&pool)
        .await?;

    Ok(())

}

async fn delete_workspace(pool: PgPool, workspace_id: Uuid) -> Result<(), sqlx::Error> {
    //same the user must be the owner( should be handle at endpoint level)
    let query = "DELETE FROM workspaces WHERE id = $1";
    sqlx::query(query)
        .bind(workspace_id)
        .execute(&pool)
        .await?;
    Ok(())
}


async fn add_members (pool : PgPool , workspace_id: Uuid , role: WorkspaceRole , members : Uuid){
    //same the user must be  the owner of the workspace to do that 
    //add single member at a time (will add multiple members adding)
    

}