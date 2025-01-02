use async_graphql::{ InputObject, SimpleObject };
use serde::{ Deserialize, Serialize };
use crate::types::graphql::DateTimeScalar;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Language {
    name: String,
    color: String,
}

#[derive(InputObject)]
pub struct LastUpdated {
    last_updated_at: DateTimeScalar,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Repository {
    last_updated_at: DateTimeScalar,
}
