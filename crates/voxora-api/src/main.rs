//create the http server
use actix_web::{HttpServer, App, web};
mod controllers;
mod extractors;
mod middleware;
use voxora_db::create_pool_connection;
use controllers::{signup, login};



#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let connections = match create_pool_connection().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("faiied to create connection to the database");
            std::process::exit(1);
        },
    };
    let pgpool  = web::Data::new(connections);
    println!("voxora-api is starting on port 8080");

    HttpServer::new(move || {
        //expects the factory to return an application instance 
        App::new()
            .app_data(pgpool.clone())
            .service(signup)
            .service(login)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
