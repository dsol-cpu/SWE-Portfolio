use std::env;
use supabase_rs::SupabaseClient;

/// Initialize a connection pool to the Supabase PostgreSQL database.
pub async fn init_database() -> SupabaseClient {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let supabase_client = SupabaseClient::new(
        env::var(crate::constants::SUPABASE_DB_URL).expect("SUPABASE_DB_URL must be set"),
        env::var(crate::constants::SUPABASE_DB_KEY).expect("SUPABASE_DB_KEY must be set")
    ).unwrap();

    supabase_client
}
