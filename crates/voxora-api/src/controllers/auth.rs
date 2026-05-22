use actix_web::{post, HttpResponse};
use voxora_core::verify_token;
use voxora_db::{get_user_data, create_pool_connection, GetUser, create_user, is_user_exist, NewUser};

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
            Ok(value) => {
                if value == true {
                    //send the error of user already exist back
                    return Err(actix_web::error::ErrorInternalServerError("User already exist"));
                }
                else {
                    //call the create user function 
                    let new_user = NewUser {
                        email: claims.email,
                        name: claims.name,
                        password_hash: claims.password_hash,
                    };
                    let db_result = create_user(new_user, &pool).await;
                    match db_result {
                        Ok(value) => {
                            return Ok(HttpResponse::Ok().body("User created successfully"));
                        }
                        Err(error) => {
                            return Err(actix_web::error::ErrorInternalServerError(error.to_string()));
                        }
                    }
                }
            },
            Err(error) => {
                return Err(actix_web::error::ErrorInternalServerError(error.to_string()))
            }
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