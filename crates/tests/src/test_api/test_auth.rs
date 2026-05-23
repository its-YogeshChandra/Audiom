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

pub async fn test_signup(){
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found in .env");

    // signup: id is None because the DB generates it
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + 3600; // 1 hour
    let claims = TestClaims {
        id: None,
        name: "marco".to_string(),
        email: "marco@example.com".to_string(),
        password: "testpassword123".to_string(),
        exp,
    };

    let token = jsonwebtoken::encode(
        &Default::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).unwrap();

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8080/signup")
        .body(token)
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
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found in .env");

    // login: same user, id still None (server looks up by email)
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + 3600; // 1 hour
    let claims = TestClaims {
        id: None,
        name: "marco".to_string(),
        email: "marco@example.com".to_string(),
        password: "testpassword123".to_string(),
        exp,
    };

    let token = jsonwebtoken::encode(
        &Default::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).unwrap();

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8080/login")
        .body(token)
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