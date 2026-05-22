use jsonwebtoken::{encode,decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;


//create the jwt token
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    id: Uuid,
    email: String,
    name: String,
}

pub fn create_token (data: Claims) -> String {
dotenv().ok();

let jwt_secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not found");
let encoding_key = EncodingKey::from_secret(jwt_secret_key.as_bytes());

//create the header
let header = Header::default();

//encode the given data 
let token = match encode(&header, &data, &encoding_key){
    Ok(token) => token,
    Err(error) => {
        eprintln!("Error creating token: {}", error);
        return error.to_string()
    },
};

//for check we can decode and check if we get the same values or not 
//but that's future me problem 
 token
}

//verify the jwt token
pub fn verify_token(token: &str) -> Result<Claims, String> {
    dotenv().ok();

    let jwt_secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not found");
    let decoding_key = DecodingKey::from_secret(jwt_secret_key.as_bytes());

    //decode the given data 
    let decoded_result = match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)){
        Ok(token) => token,
        Err(error) => match error.kind() {
            ErrorKind::InvalidToken => panic!("Invalid token"),
            _ => panic!("Invalid token")
        }
    };

    Ok(decoded_result.claims)
     
} 