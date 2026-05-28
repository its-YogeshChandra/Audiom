//create the http server
use actix_web::{HttpServer, App, web};
mod controllers;
mod extractors;
mod middleware;

use voxora_db::create_pool_connection;
use controllers::{
    signup, login,
    create_workspace_endpoint, get_workspaces_endpoint, get_workspace_by_slug_endpoint,
    update_workspace_endpoint, delete_workspace_endpoint,
    get_workspace_members_endpoint, add_workspace_member_endpoint,
    remove_workspace_member_endpoint, change_workspace_member_role_endpoint,
    create_project_controller, get_projects_by_workspace_controller,
    get_project_by_id_controller, update_project_by_id_controller,
    delete_project_by_id_controller,
};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let connections = match create_pool_connection().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("failed to create connection to the database");
            std::process::exit(1);
        },
    };
    let pgpool = web::Data::new(connections);
    println!("voxora-api is starting on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(pgpool.clone())
            // auth
            .service(signup)
            .service(login)
            // workspaces
            .service(create_workspace_endpoint)
            .service(get_workspaces_endpoint)
            .service(get_workspace_by_slug_endpoint)
            .service(update_workspace_endpoint)
            .service(delete_workspace_endpoint)
            // workspace members
            .service(get_workspace_members_endpoint)
            .service(add_workspace_member_endpoint)
            .service(remove_workspace_member_endpoint)
            .service(change_workspace_member_role_endpoint)
            // projects
            .service(create_project_controller)
            .service(get_projects_by_workspace_controller)
            .service(get_project_by_id_controller)
            .service(update_project_by_id_controller)
            .service(delete_project_by_id_controller)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
