use anyhow::{Context, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Create a PostgreSQL connection pool
pub async fn create_postgres_pool(database_url: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(50)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await
        .context("Failed to create PostgreSQL connection pool")
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("Failed to run database migrations")
}

/// Health check for PostgreSQL connection
pub async fn postgres_health_check(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .context("PostgreSQL health check failed")?;
    
    tracing::debug!("PostgreSQL health check passed");
    Ok(())
} 