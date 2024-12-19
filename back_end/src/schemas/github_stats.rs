use async_graphql::SimpleObject;
use serde::{ Deserialize, Serialize };
use crate::types::graphql::DateTimeScalar;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Repository {
    name: String,
    description: Option<String>,
    languages: Option<Vec<Language>>,
    last_updated_at: DateTimeScalar,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Language {
    name: String,
    color: String,
}
