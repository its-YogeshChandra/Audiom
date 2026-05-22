use actix_web::{post, web, HttpResponse};

//signup endpoint 
#[post("/signup")]
pub async fn signup() -> Result<HttpResponse, actix_web::Error> {
    //take the token from the frontend(generated through clerk) and verify it 

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