//create the http server
use actix_web::{HttpServer, App, web};
mod controllers;
mod extractors;
mod middleware;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("voxora-api is starting on port 8080");

    HttpServer::new(move || {
        //expects the factory to return an application instance 
        App::new()
            .route("/", web::get().to(|| async { "voxora-api is running" }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
