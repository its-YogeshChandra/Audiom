use actix_web::{post, HttpResponse};
use voxora_core::verify_token;
use voxora_db::{get_user_data, create_pool_connection, GetUser, create_user, is_user_exist};

//signup endpoint 
#[post("/signup")]
pub async fn signup(jwt_token: String) -> Result<HttpResponse, actix_web::Error> {
    //take the token from the frontend(generated through clerk) and verify it 

    match verify_token(&jwt_token) {
    Ok(claims) => {
        tracing::info!("Token verified successfully: {:#?}", claims);

        //serealize the data we get into getuserstruct
        let user_data = GetUser::Email(claims.email);
        //we move further inside this only 
        let pool = create_pool_connection().await.map_err(|error| actix_web::error::ErrorInternalServerError(error.to_string()))?;
        let db_result = is_user_exist(&pool, user_data).await;

        match db_result {
            Ok(value) => {},
            Err(error) => {}
        }

    } 

    Err(error) => {
        //handling should be done here I guess 
        tracing::error!("Failed to verify token: {}", error);
      return Err(actix_web::error::ErrorInternalServerError(error.to_string()));
    }
    }


}


//login endpoint 
#[post("/login")]
pub async fn login() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("login"))
}