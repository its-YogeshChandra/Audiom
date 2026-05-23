//use the reqwest to test the auth endpoints of api's 
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserData {
    name: String, 
    email: String, 
    avatar_url: String,
}

pub async fn test_signup(){
    //take the token from the clerk (local for now) and test the signup endpoint of api

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8000/signup")
        .json(&UserData {
            name: "John Doe".to_string(),
            email: "[EMAIL_ADDRESS]".to_string(),
            avatar_url: "https://example.com/avatar.jpg".to_string(),
        })
        .send()
        .await
        .unwrap();
    
    println!("{:#?}", response);
    
}