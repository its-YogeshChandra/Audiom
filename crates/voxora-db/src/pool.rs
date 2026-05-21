use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub async fn create_pool_connection() -> Result<PgPool, sqlx::Error> {



    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect("postgres://user:password@localhost:5432/database")
        .await?;

    Ok(pool)
}