//use the reqwest to test the auth endpoints of api's 
use reqwest;
use serde::{Deserialize, Serialize};
use jsonwebtoken;
use tracing;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    name: String, 
    email: String, 
    avatar_url: String,
}

pub async fn test_signup(){
    //take the token from the clerk (local for now) and test the signup endpoint of api

    //create the jwt token out of the userdata 
    let user_data = UserData {
        name: "John Doe".to_string(),
        email: "[EMAIL_ADDRESS]".to_string(),
        avatar_url: "https://example.com/avatar.jpg".to_string(),
    };

    //use jwt to get the token 
    let token = jsonwebtoken::encode(&Default::default(), &user_data, &jsonwebtoken::EncodingKey::from_secret("secret".as_bytes())).unwrap();
    tracing::info!("Token: {:#?}", token);

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8000/signup")
        .json(&user_data)
        .send()
        .await
        .unwrap();
    
    println!("{:#?}", response);
    
}


pub async fn test_login(){
    //take the token from the clerk (local for now) and test the login endpoint of api

    //create the jwt token out of the userdata 
    let user_data = UserData {
        name: "John Doe".to_string(),
        email: "[EMAIL_ADDRESS]".to_string(),
        avatar_url: "https://example.com/avatar.jpg".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8000/login")
        .json(&user_data)
        .send()
        .await
        .unwrap();
    
    println!("{:#?}", response);
    
}
         