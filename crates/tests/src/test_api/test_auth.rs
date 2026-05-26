//use the reqwest to test the auth endpoints of api's 
use reqwest;
use serde::{Deserialize, Serialize};
use jsonwebtoken;
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

// Must match Claims struct in voxora-core/src/auth/jwt.rs exactly
#[derive(Serialize, Deserialize)]
pub struct TestClaims {
    id: Option<Uuid>,
    email: String,
    name: String,
    password: String,
    exp: usize,
}

/// Generate a JWT token for testing
pub fn make_token(id: Option<Uuid>, email: &str, name: &str, password: &str) -> String {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found in .env");

    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + 3600;
    let claims = TestClaims {
        id,
        name: name.to_string(),
        email: email.to_string(),
        password: password.to_string(),
        exp,
    };

    jsonwebtoken::encode(
        &Default::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).unwrap()
}

pub async fn test_signup(){
    dotenv().ok();

    let token = make_token(None, "marco@example.com", "marco", "testpassword123");
    println!("[SIGNUP] token: {}", token);

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8080/signup")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[SIGNUP] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[SIGNUP] error: {:#?}", error);
        }
    }
}


pub async fn test_login(){
    dotenv().ok();

    let token = make_token(None, "marco@example.com", "marco", "testpassword123");

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8080/login")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("[LOGIN] status: {} | body: {}", status, body);
        }
        Err(error) => {
            println!("[LOGIN] error: {:#?}", error);
        }
    }
}