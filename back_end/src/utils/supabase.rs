use std::env;
use deadpool::{ managed::QueueMode, Runtime };
use deadpool_postgres::{ tokio_postgres::NoTls, Config, Pool, PoolConfig, Timeouts };
use serde::de::DeserializeOwned;

use crate::{
    constants::{
        SUPABASE_DB_HOST,
        SUPABASE_DB_NAME,
        SUPABASE_DB_PASSWORD,
        SUPABASE_DB_PORT,
        SUPABASE_DB_USER,
    },
    types::error::ApiError,
};

#[derive(Clone)]
pub struct SupabaseClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl SupabaseClient {
    pub fn new() -> Result<Self, ApiError> {
        let project_ref = env
            ::var("SUPABASE_PROJECT_REF")
            .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_PROJECT_REF: {}", e)))?;
        let api_key = env
            ::var("SUPABASE_ANON_KEY")
            .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_ANON_KEY: {}", e)))?;

        let base_url = format!("https://{}.supabase.co/rest/v1", project_ref);

        Ok(Self {
            client: Client::new(),
            base_url,
            api_key,
        })
    }

    pub async fn rpc<T: DeserializeOwned>(&self, function_name: &str) -> Result<T, ApiError> {
        let url = format!("{}/rpc/{}", self.base_url, function_name);

        let response = self.client
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send().await
            .map_err(|e| ApiError::ExternalApiError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::ExternalApiError(error_text));
        }

        response.json::<T>().await.map_err(|e| ApiError::SerializationError(e.to_string()))
    }

    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self.client
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(body)
            .send().await
            .map_err(|e| ApiError::ExternalApiError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::ExternalApiError(error_text));
        }

        response.json::<T>().await.map_err(|e| ApiError::SerializationError(e.to_string()))
    }
}

// Initialize database connection pool
pub async fn init_database() -> Result<SupabaseClient, ApiError> {
    let db_name = env
        ::var(SUPABASE_DB_NAME)
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_NAME: {}", e)))?;
    let db_user = env
        ::var(SUPABASE_DB_USER)
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_USER: {}", e)))?;
    let db_password = env
        ::var(SUPABASE_DB_PASSWORD)
        .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_PASSWORD: {}", e)))?;

    // Configure the Deadpool for PostgreSQL
    let mut cfg = Config::new();
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

    Ok(SupabaseClient::new(pool))
}
