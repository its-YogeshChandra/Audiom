use actix_web::{post, web, HttpResponse};
use voxora_core::verify_token;
use voxora_db::get_user_data;

//signup endpoint 
#[post("/signup")]
pub async fn signup(jwt_token: String) -> Result<HttpResponse, actix_web::Error> {
    //take the token from the frontend(generated through clerk) and verify it 
    let result = match verify_token(&jwt_token) {
    Ok(claims) => {
        tracing::info!("Token verified successfully: {:#?}", claims);
        claims
    }
    Err(error) => {
        //handling should be done here I guess 
        tracing::error!("Failed to verify token: {}", error);
    }
    };

    //call the database function to check for the user info we get 
    let db_result = get_user_data(pool, user_data)
    


    //check for the credentials in the database 
        //if present then send error 
        //if not then call the create user function of the database 

    Ok(HttpResponse::Ok().body("signup"))
}


//login endpoint 
#[post("/login")]
pub async fn login() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("login"))
}