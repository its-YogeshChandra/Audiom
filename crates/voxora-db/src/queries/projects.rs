use uuid::Uuid;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
//check on this package 
use time::UtcDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id:           Uuid,
    pub workspace_id: Uuid,
    pub name:         String,
    pub description:  Option<String>,
    pub artwork_url:  Option<String>,
    pub rss_slug:     Option<String>,
    pub created_at:   UtcDateTime,
}



#[derive(Debug, Deserialize)]
pub struct NewProject {
    pub name:         String,
    pub description:  Option<String>,
    pub artwork_url:  Option<String>,
    pub rss_slug:     Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct UpdateProject {
    pub name:         Option<String>,
    pub description:  Option<String>,
    pub artwork_url:  Option<String>,
    pub rss_slug:     Option<String>,
}

pub async fn create_project(pool: &PgPool, workspace_id: Uuid, new_project: NewProject) -> Result<Project, sqlx::Error> {
    
    sqlx::query_as!(
        Project,
        r#"
        INSERT INTO projects (workspace_id, name, description, artwork_url, rss_slug)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, workspace_id, name, description, artwork_url, rss_slug, created_at
        "#,
        workspace_id,
        new_project.name,
        new_project.description,
        new_project.artwork_url,
        new_project.rss_slug
    )
    .fetch_one(pool)
    .await
}

pub async fn get_project_by_workspace_id(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as!(
        Project,
        r#"
        SELECT id, workspace_id, name, description, artwork_url, rss_slug, created_at
        FROM projects
        WHERE workspace_id = $1
        ORDER BY created_at DESC
        "#,
        workspace_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_project_by_id(pool: &PgPool, project_id: Uuid) -> Result<Project, sqlx::Error> {
    sqlx::query_as!(
        Project,
        r#"
        SELECT id, workspace_id, name, description, artwork_url, rss_slug, created_at
        FROM projects
        WHERE id = $1
        "#,
        project_id
    )
    .fetch_one(pool)
    .await
}


pub async fn delete_project_by_id(pool: &PgPool, project_id: Uuid) -> Result<u64, sqlx::Error> {
    //only owner can do this so the controller has to check for the owner access
    let result = sqlx::query!(
        r#"
        DELETE FROM projects
        WHERE id = $1
        "#,
        project_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}


pub async fn update_project_by_id(pool: &PgPool, project_id: Uuid, update_project: UpdateProject) -> Result<Project, sqlx::Error> {
    //only owner and admin can do this so controller check required 
    let result = sqlx::query_as!(
        Project,
        r#"
        UPDATE projects
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            artwork_url = COALESCE($3, artwork_url),
            rss_slug = COALESCE($4, rss_slug)
        WHERE id = $5
        RETURNING id, workspace_id, name, description, artwork_url, rss_slug, created_at
        "#,
        update_project.name,
        update_project.description,
        update_project.artwork_url,
        update_project.rss_slug,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}