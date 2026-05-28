//api for the projects
use actix_web::{get, post, put, web, Responder, HttpResponse};
use voxora_db::{create_project, delete_project_by_id, update_project_by_id, NewProject, UpdateProject, get_project_by_workspace_id};
use sqlx::PgPool;
use uuid::Uuid;

//create project endpoint 
#[post("/project")]
pub async fn create_project_controller(pool: web::Data<PgPool>, workspace_id: String, new_project: web::Json<NewProject>) -> Result<HttpResponse, actix_web::Error> {
    //prase workspace_id to uuid
    let workspace_id = Uuid::parse_str(&workspace_id).map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    
    let project = create_project(&pool,workspace_id, new_project.into_inner()).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Created().json(project))
}

#[get("/project/{workspace_id}")]
pub async fn get_project_by_workspace_id_controller(pool: web::Data<PgPool>, workspace_id: String) -> Result<HttpResponse, actix_web::Error> {
    let workspace_id = Uuid::parse_str(&workspace_id).map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    let project = get_project_by_workspace_id(&pool,workspace_id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(project))
}

#[put("/project/{project_id}")]
pub async fn update_project_by_id_controller(pool: web::Data<PgPool>, project_id: String, update_project: web::Json<UpdateProject>) -> Result<HttpResponse, actix_web::Error> {
    let project_id = Uuid::parse_str(&project_id).map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    let project = update_project_by_id(&pool,project_id, update_project.into_inner()).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(project))
}