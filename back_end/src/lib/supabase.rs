use reqwest::{ Client, Response };
use serde::de::DeserializeOwned;
use std::env;
use std::error::Error;
use actix_web::web;

// Generic error type for Supabase operations
#[derive(Debug)]
pub enum SupabaseError {
    RequestError(reqwest::Error),
    NotFound,
    // Add more error types as needed
}

impl std::fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupabaseError::RequestError(e) => write!(f, "Supabase request error: {}", e),
            SupabaseError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl Error for SupabaseError {}

impl From<reqwest::Error> for SupabaseError {
    fn from(error: reqwest::Error) -> Self {
        SupabaseError::RequestError(error)
    }
}

impl actix_web::error::ResponseError for SupabaseError {}

#[derive(Clone)]
pub struct SupabaseClient {
    client: Client,
    url: String,
    api_key: String,
}

impl SupabaseClient {
    pub fn new() -> Self {
        let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let api_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");

        Self {
            client: Client::new(),
            url,
            api_key,
        }
    }

    // Generic select query builder
    pub fn from<T: DeserializeOwned>(&self, table: &str) -> QueryBuilder<T> {
        QueryBuilder::new(
            self.client.clone(),
            self.url.clone(),
            self.api_key.clone(),
            table.to_string()
        )
    }

    // Helper method for direct queries
    pub async fn select<T: DeserializeOwned>(
        &self,
        table: &str,
        query_params: Option<Vec<(&str, String)>>
    ) -> Result<Vec<T>, SupabaseError> {
        let mut request = self.client
            .get(&format!("{}/rest/v1/{}", self.url, table))
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key));

        if let Some(params) = query_params {
            request = request.query(&params);
        }

        let response: Response = request.send().await?;
        let data = response.json::<Vec<T>>().await?;
        Ok(data)
    }

    // Insert data
    pub async fn insert<T: DeserializeOwned, U: serde::Serialize>(
        &self,
        table: &str,
        data: &U
    ) -> Result<T, SupabaseError> {
        let response = self.client
            .post(&format!("{}/rest/v1/{}", self.url, table))
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Prefer", "return=representation")
            .json(data)
            .send().await?
            .json::<T>().await?;

        Ok(response)
    }

    // Update data
    pub async fn update<T: DeserializeOwned, U: serde::Serialize>(
        &self,
        table: &str,
        id: i32,
        data: &U
    ) -> Result<T, SupabaseError> {
        let response = self.client
            .patch(&format!("{}/rest/v1/{}", self.url, table))
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Prefer", "return=representation")
            .query(&[("id", format!("eq.{}", id))])
            .json(data)
            .send().await?
            .json::<T>().await?;

        Ok(response)
    }

    // Delete data
    pub async fn delete(&self, table: &str, id: i32) -> Result<(), SupabaseError> {
        self.client
            .delete(&format!("{}/rest/v1/{}", self.url, table))
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .query(&[("id", format!("eq.{}", id))])
            .send().await?;

        Ok(())
    }
}

// Query builder for more complex queries
pub struct QueryBuilder<T> {
    client: Client,
    url: String,
    api_key: String,
    table: String,
    filters: Vec<(String, String)>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> QueryBuilder<T> {
    fn new(client: Client, url: String, api_key: String, table: String) -> Self {
        Self {
            client,
            url,
            api_key,
            table,
            filters: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn eq(mut self, column: &str, value: &str) -> Self {
        self.filters.push((column.to_string(), format!("eq.{}", value)));
        self
    }

    pub fn like(mut self, column: &str, value: &str) -> Self {
        self.filters.push((column.to_string(), format!("like.{}", value)));
        self
    }

    pub fn in_array(mut self, column: &str, values: Vec<String>) -> Self {
        self.filters.push((column.to_string(), format!("in.({})", values.join(","))));
        self
    }

    pub fn order(mut self, column: &str, ascending: bool) -> Self {
        let direction = if ascending { "asc" } else { "desc" };
        self.filters.push(("order".to_string(), format!("{}.{}", column, direction)));
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.filters.push(("limit".to_string(), limit.to_string()));
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.filters.push(("offset".to_string(), offset.to_string()));
        self
    }

    pub async fn execute(self) -> Result<Vec<T>, SupabaseError> {
        let mut request = self.client
            .get(&format!("{}/rest/v1/{}", self.url, self.table))
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key));

        for (key, value) in self.filters {
            request = request.query(&[(key.as_str(), value.as_str())]);
        }

        let response = request.send().await?;
        let data = response.json::<Vec<T>>().await?;
        Ok(data)
    }

    pub async fn execute_single(self) -> Result<Option<T>, SupabaseError> {
        let mut result = self.limit(1).execute().await?;
        Ok(result.pop())
    }
}
