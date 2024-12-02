use async_graphql::SimpleObject;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Repository {
    name: String,
    description: Option<String>,
    languages: Option<Vec<Language>>,
    last_updated_at: crate::types::graphql::DateTimeScalar,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Language {
    name: String,
    color: String,
}
