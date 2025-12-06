use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn connect(database_url: &str) -> Result<DbPool, sqlx::Error> {
    tracing::info!("Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .connect(database_url)
        .await?;

    tracing::info!("✅ Database connected");
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;
    tracing::info!("✅ Migrations completed");
    Ok(())
}
