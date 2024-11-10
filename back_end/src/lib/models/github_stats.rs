use serde::Serialize;

#[derive(Serialize)]
pub struct GithubStats {
    pub name: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub cached: String,
    pub cache_age: String,
}
