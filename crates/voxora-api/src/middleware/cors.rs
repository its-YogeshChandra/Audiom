use actix_cors::{Cors, CorsError};
use actix_web::{http};


pub async fn cors_middleware() -> Cors{
    Cors::default()
    .allowed_origin("http://localhost:3000")
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    .supports_credentials() //can't use the wildcard origin if use the cors 
    .max_age(3600)
}