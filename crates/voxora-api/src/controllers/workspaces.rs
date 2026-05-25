//workspace controller for all the workspace endpoints 
use actix_web::{get , post, put , delete, Responder,web, HttpResponse};
use uuid::Uuid;


#[post("/workspaces")]
pub async fn create_workspace() -> impl Responder {
    HttpResponse::Ok().body("Workspace created successfully")
}  

#[get   ("/workspaces")]
pub async fn get_workspace() -> impl Responder {
    HttpResponse::Ok().body("Workspace fetched successfully")
}

#[get("/workspaces/{slug}")]
pub async fn get_workspace_by_slug(path: web::Path<(String)>) -> impl Responder {
    HttpResponse::Ok().body("Workspace fetched successfully")
}

#[put("/workspaces/{workspace_id}")]
pub async fn update_workspace(path: web::Path<(Uuid)>) -> impl Responder {
    HttpResponse::Ok().body("Workspace updated successfully")
}

#[get("/workspace/{workspace_id}/members")]
pub async fn get_workspace_members(path: web::Path<(Uuid)>) -> impl Responder {
    HttpResponse::Ok().body("Workspace members fetched successfully")
}