use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest};
use uuid::Uuid;
use sqlx::PgPool;
use voxora_db::{create_room, update_room, update_room_status, delete_room, get_room_by_id, get_rooms_by_project_id, NewRoom, Room, UpdateRoom};
use crate::extractors::{
    extract_user_id, require_owner_or_admin, require_member,
};

#[post("/workspace/{id}/room")]
pub async fn create_room_endpoint(
    req : HttpRequest ,
    pool : web::Data<PgPool>,
    path : web::Path<(Uuid, )>,
    payload : web::Json<NewRoom>
) -> Result<HttpResponse , actix_web::Error>{
   //have to implement auth and rbac functions 
   let user_id = extract_user_id(&req)?;
   let workspace_id = path.into_inner().0;

   require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;
   
   let room = create_room(pool.as_ref(), payload.into_inner()).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
   Ok(HttpResponse::Ok().json(room))
}

#[get("/workspace/{id}/room")]
pub async fn get_room_endpoint(
    req : HttpRequest , 
    pool : web::Data<PgPool> , 
    path : web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse , actix_web::Error>{
   //have to implement auth and rbac functions 
    let user_id = extract_user_id(&req)?;
    let (workspace_id, room_id) = path.into_inner(); 

    require_member(pool.as_ref(), workspace_id, user_id).await?;

    let room = get_room_by_id(pool.as_ref(), room_id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(room))
}

#[get("/workspace/{id}/room")]
pub async fn get_rooms_endpoint(
    req : HttpRequest,
    pool : web::Data<PgPool> , 
    path : web::Path<(Uuid , Uuid)>,
) -> Result<HttpResponse , actix_web::Error>{
   //have to implement auth and rbac functions  
   let user_id = extract_user_id(&req)?;
   let (workspace_id, project_id) = path.into_inner();

   require_member(pool.as_ref(), workspace_id, user_id).await?;
   
   let rooms = get_rooms_by_project_id(pool.as_ref(), project_id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
   Ok(HttpResponse::Ok().json(rooms))
}

#[put("/workspace/{id}/room")]
pub async fn update_room_endpoint(
    req : HttpRequest,
    pool : web::Data<PgPool>,
    path : web::Path<(Uuid , Uuid)>,
    payload : web::Json<UpdateRoom>,
) -> Result<HttpResponse , actix_web::Error>{
   //have to implement auth and rbac functions 
   let user_id = extract_user_id(&req)?;
   let (workspace_id, room_id) = path.into_inner();
   
   require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;
   
   let room = update_room(pool.as_ref(), room_id, payload.into_inner()).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(room))
}


#[put("/workspace/{id}/room/{room_id}/status")]
pub async fn update_room_status_endpoint(
    req : HttpRequest,
    pool : web::Data<PgPool>,
    path : web::Path<(Uuid, Uuid)>,
    payload : web::Json<String>,
) -> Result<HttpResponse , actix_web::Error>{
   //have to implement auth and rbac functions 
   let user_id = extract_user_id(&req)?;
   let (workspace_id, room_id) = path.into_inner();
   
   require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

   let status = payload.into_inner();
   let room = update_room_status(pool.as_ref(), room_id, status).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(room))
}


#[delete("/workspace/{id}/room")]
pub async fn delete_room_endpoint(
    req : HttpRequest,
    pool : web::Data<PgPool> , 
    path : web::Path<(Uuid ,Uuid)>,
) -> Result<HttpResponse , actix_web::Error>{
   //have to implement auth and rbac functions  
    let user_id = extract_user_id(&req)?;
    let (workspace_id, room_id) = path.into_inner();
   
    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;
    
    let room = delete_room(pool.as_ref(), room_id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(room))
}

