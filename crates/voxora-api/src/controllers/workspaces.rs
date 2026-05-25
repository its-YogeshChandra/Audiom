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

#[post("/workspace/{workspace_id}/members")]
pub async fn add_workspace_members(path: web::Path<(Uuid)>) -> impl Responder {
    HttpResponse::Ok().body("Workspace members added successfully")
}

#[delete("/workspace/{workspace_id}/members/{user_id}")]
pub async fn remove_workspace_members(path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    HttpResponse::Ok().body("Workspace members removed successfully")
}

#[put("/workspace/{workspace_id}/members/{user_id}/role")]
pub async fn change_workspace_members_role(path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    HttpResponse::Ok().body("Workspace members role changed successfully")
}