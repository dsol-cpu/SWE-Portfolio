use std::env;
use deadpool::managed::{ Pool, QueueMode };
use deadpool_postgres::{
    Config as DeadpoolConfig,
    PoolConfig,
    SslMode,
    Manager,
    tokio_postgres as pg,
};
use tokio_postgres_rustls::MakeRustlsConnect;
use rustls::{ pki_types::CertificateDer, ClientConfig, RootCertStore };
use webpki_roots;
use crate::{
    constants::{ SUPABASE_DB_NAME, SUPABASE_DB_USER, SUPABASE_DB_PASSWORD },
    types::error::ApiError,
};

pub async fn init_database() -> Result<Pool<Manager>, ApiError> {
    let mut cfg = pg::Config::new();
    cfg.dbname(
        env
            ::var(SUPABASE_DB_NAME)
            .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_NAME: {}", e)))?
    );
    cfg.user(
        env
            ::var(SUPABASE_DB_USER)
            .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_USER: {}", e)))?
    );
    cfg.password(
        env
            ::var(SUPABASE_DB_PASSWORD)
            .map_err(|e| ApiError::ConfigError(format!("Missing SUPABASE_DB_PASSWORD: {}", e)))?
    );
    cfg.host("db.supabase.co");
    cfg.port(5432);
    cfg.ssl_mode(SslMode::Require.into());

    // Convert TrustAnchor to CertificateDer
    let root_certs: Vec<CertificateDer<'static>> = webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .map(|trust_anchor| { CertificateDer::from(trust_anchor.subject_public_key_info.to_vec()) })
        .collect();

    let mut root_cert_store = RootCertStore::empty();
    for cert in root_certs {
        root_cert_store
            .add(cert)
            .map_err(|e| ApiError::ConfigError(format!("Failed to add root certificate: {}", e)))?;
    }

    let client_config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let make_tls = MakeRustlsConnect::new(client_config);
    let mut deadpool_config = DeadpoolConfig::new();
    deadpool_config.dbname = cfg.get_dbname().map(String::from);
    deadpool_config.user = cfg.get_user().map(String::from);
    deadpool_config.password = cfg
        .get_password()
        .map(|pw| std::str::from_utf8(pw).expect("Password should be valid UTF-8").to_string());

    deadpool_config.pool = Some(PoolConfig {
        max_size: 16,
        queue_mode: QueueMode::Lifo,
        timeouts: deadpool::managed::Timeouts {
            wait: Some(std::time::Duration::from_secs(15)),
            create: Some(std::time::Duration::from_secs(5)),
            recycle: Some(std::time::Duration::from_secs(5)),
        },
    });

    deadpool_config
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), make_tls)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))
}
