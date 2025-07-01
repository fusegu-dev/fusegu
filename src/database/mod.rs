pub mod postgres;
pub mod redis;

pub use postgres::*;
pub use redis::*;

// Re-export Database as DatabasePool for backward compatibility
pub type DatabasePool = Database;

use anyhow::Result;
use sqlx::PgPool;
use ::redis::Client as RedisClient;

/// Database connection manager for the application
#[derive(Debug, Clone)]
pub struct Database {
    pub pg_pool: PgPool,
    pub redis_client: RedisClient,
}

impl Database {
    /// Create a new database connection manager
    pub async fn new(database_url: &str, redis_url: &str) -> Result<Self> {
        let pg_pool = create_postgres_pool(database_url).await?;
        let redis_client = create_redis_client(redis_url)?;

        Ok(Self {
            pg_pool,
            redis_client,
        })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        run_migrations(&self.pg_pool).await
    }

    /// Get a PostgreSQL connection from the pool
    pub fn postgres(&self) -> &PgPool {
        &self.pg_pool
    }

    /// Get a Redis connection
    pub fn redis(&self) -> &RedisClient {
        &self.redis_client
    }
} 