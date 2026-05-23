use jsonwebtoken::{encode,decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;


//create the jwt token
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub id: Option<Uuid>,
    pub email: String,
    pub name: String,
    pub password: String,
    pub exp: usize,  // expiration timestamp (required by JWT spec)
}

//verify the jwt token
pub fn verify_token(token: &str) -> Result<Claims, String> {
    dotenv().ok();

    let jwt_secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not found");
    let decoding_key = DecodingKey::from_secret(jwt_secret_key.as_bytes());

    //decode the given data 
    let decoded_result = match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)){
        Ok(token) => token,
        Err(error) => return Err(format!("Token verification failed: {}", error))
    };

    Ok(decoded_result.claims)
     
} 