use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use dotenvy::dotenv;

pub async fn create_pool_connection() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    //extract the database url from the env file
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pool)
}