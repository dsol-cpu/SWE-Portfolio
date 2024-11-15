use std::env;
use deadpool::{ managed::QueueMode, Runtime };
use deadpool_postgres::{ tokio_postgres::NoTls, Config, Pool, PoolConfig, Timeouts };

use crate::types::error::ApiError;

// Config structure to hold the pool
#[derive(Clone)]
pub struct SupabaseConfig {
    db_pool: Pool,
}

impl SupabaseConfig {
    // Get the pool
    pub fn get_pool(&self) -> &Pool {
        &self.db_pool
    }

    // Create a new instance
    pub fn new(pool: Pool) -> Self {
        Self { db_pool: pool }
    }
}

// Initialize database connection pool
pub async fn init_database() -> Result<SupabaseConfig, ApiError> {
    let db_host = env
        ::var("SUPABASE_DB_HOST")
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_HOST: {}", e)))?;
    let db_port = env
        ::var("SUPABASE_DB_PORT")
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_PORT: {}", e)))?
        .parse::<u16>()
        .map_err(|e| ApiError::ConfigError(format!("Invalid SUPABASE_DB_PORT: {}", e)))?;
    let db_name = env
        ::var("SUPABASE_DB_NAME")
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_NAME: {}", e)))?;
    let db_user = env
        ::var("SUPABASE_DB_USER")
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_USER: {}", e)))?;
    let db_password = env
        ::var("SUPABASE_DB_PASSWORD")
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_PASSWORD: {}", e)))?;

    // Configure the Deadpool for PostgreSQL
    let mut cfg = Config::new();
    cfg.host = Some(db_host);
    cfg.port = Some(db_port);
    cfg.dbname = Some(db_name);
    cfg.user = Some(db_user);
    cfg.password = Some(db_password);
    cfg.ssl_mode = Some(deadpool_postgres::SslMode::Require); // Use SSL mode if needed

    // Configure pool settings
    cfg.pool = Some(PoolConfig {
        max_size: 16,
        queue_mode: QueueMode::Lifo,
        timeouts: Timeouts {
            wait: Some(std::time::Duration::from_secs(15)),
            create: Some(std::time::Duration::from_secs(5)),
            recycle: Some(std::time::Duration::from_secs(5)),
        },
    });

    // Create the pool using `NoTls` for an unencrypted connection
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(SupabaseConfig::new(pool))
}
