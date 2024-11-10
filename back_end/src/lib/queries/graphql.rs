use serde::Serialize;

// GraphQL query struct
#[derive(Serialize)]
pub struct GraphQLQuery {
    query: String,
    variables: Option<serde_json::Value>,
}
