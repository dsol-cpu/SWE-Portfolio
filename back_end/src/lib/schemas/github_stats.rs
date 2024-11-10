use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct GithubStats {
    pub name: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub cached: bool, // Added to indicate if data came from cache
    pub cache_age: DateTime<Utc>, // Optional: time since last cache update in seconds
}

#[derive(Serialize, Deserialize)]
pub struct CachedStats {
    pub last_updated: DateTime<Utc>,
    pub stats: GithubStats,
}

pub struct GithubStatsGraphQLSchema {}
