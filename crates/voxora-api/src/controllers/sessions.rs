use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use voxora_db::{Session, NewSession, UpdateSession, create_session , get_session_by_id , get_sessions_by_room_id , update_session , delete_session};
use crate::extractors::{extract_user_id, require_owner_or_admin};
use uuid::Uuid;


//create session 
#[post("workspace/rooms/sessions")]
pub async fn create_session_controller(
    req : HttpRequest,
    pool : web::Data<PgPool>,
    path : web::Path<(Uuid,Uuid )>,
    body : web::Json<NewSession>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, room_id) = path.into_inner();

    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    let session = create_session(pool.as_ref(), body.into_inner())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(session))
}


#[get("workspace/{workspace_id}/rooms/{room_id}/sessions")]
pub async fn get_session_controller(
    req : HttpRequest,
    pool : web::Data<PgPool>,
    path : web::Path<(Uuid,Uuid)>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_user_id(&req)?;
    let (workspace_id, room_id) = path.into_inner();

    require_owner_or_admin(pool.as_ref(), workspace_id, user_id).await?;

    let session = get_session_by_id(pool.as_ref(), room_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(session))
}
