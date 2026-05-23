use actix_web::{post, HttpResponse, web};
use voxora_core::{verify_token};
use voxora_db::{get_user_data, create_pool_connection, GetUser, create_user, is_user_exist, NewUser};
use sqlx::PgPool;

//signup endpoint 
//will take the decrypted user data from the auth middleware ( future me problem)
#[post("/signup")]
pub async fn signup(jwt_token: String, pgpool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    //take the token from the frontend(generated through clerk) and verify it 

    match verify_token(&jwt_token) {
    Ok(claims) => {
        tracing::info!("Token verified successfully: {:#?}", claims);

        //serealize the data we get into getuserstruct
        let user_data = GetUser::Email(claims.email.clone());

        //we move further inside this only 
        let pool = pgpool.as_ref();
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
                        password_hash: claims.password,
                    };
                    let db_result = create_user(new_user, &pool).await;
                    match db_result {
                        Ok(_) => {
                            //has to send the user created without password hash in the response (future me problem)

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
pub async fn login(jwt_token: String, pgpool : web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    // 1 Verify the clerk jwt
    let claims = verify_token(&jwt_token)
        .map_err(|e| {
            tracing::error!("Failed to verify token: {}", e);
            actix_web::error::ErrorUnauthorized("Invalid or expired token")
        })?;

    tracing::info!("Token verified successfully: {:#?}", claims);

    // 2. Connect to DB
    let pool = pgpool.as_ref();
    let user_data = GetUser::Email(claims.email.clone());
    let user = get_user_data(&pool, user_data).await
        .map_err(|e| {
            tracing::error!("Failed to fetch user: {}", e);
            actix_web::error::ErrorNotFound("User not found. Please sign up first.")
        })?;

    // 4. Return the user data
    tracing::info!("User logged in successfully: {:#?}", user);
    Ok(HttpResponse::Ok().body(format!("Welcome back, {}", user.name)))
}