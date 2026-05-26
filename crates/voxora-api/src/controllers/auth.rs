use actix_web::{post, HttpRequest, HttpResponse, web};
use voxora_core::{verify_token};
use voxora_db::{get_user_data, create_pool_connection, GetUser, create_user, is_user_exist, NewUser};
use sqlx::PgPool;

/// Extract Bearer token from the Authorization header
fn extract_bearer_token(req: &HttpRequest) -> Result<String, actix_web::Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?
        .to_str()
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(actix_web::error::ErrorUnauthorized("Authorization header must start with 'Bearer '"));
    }

    Ok(auth_header["Bearer ".len()..].to_string())
}

//signup endpoint 
//will take the decrypted user data from the auth middleware ( future me problem)
#[post("/signup")]
pub async fn signup(req: HttpRequest, pgpool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    //take the token from the frontend(generated through clerk) and verify it 
    let jwt_token = extract_bearer_token(&req)?;

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
                        Ok(user) => {
                            return Ok(HttpResponse::Created().json(serde_json::json!({
                                "id": user.id,
                                "email": user.email,
                                "name": user.name,
                            })));
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
pub async fn login(req: HttpRequest, pgpool : web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    // 1 Verify the clerk jwt
    let jwt_token = extract_bearer_token(&req)?;
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