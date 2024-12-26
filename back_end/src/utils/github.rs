use actix_web::{ web, FromRequest, HttpRequest, HttpResponse };
use async_graphql::{ InputObject, SimpleObject, Context, Schema, Object, Result };
use reqwest::Client;
use std::sync::Arc;
use crate::{ constants::GITHUB_GRAPHQL_API_URL, schemas::github_stats::LastUpdated };

pub struct GithubClient {
    client: Client,
    token: String,
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub async fn get_repository(&self, owner: &str, name: &str) -> Result<LastUpdated> {
        let query = format!(
            r#"
            query {{
                repository(owner: "{}", name: "{}") {{
                    name
                    description
                    stargazerCount
                    forkCount
                    primaryLanguage {{
                        name
                        color
                    }}
                }}
            }}
            "#,
            owner,
            name
        );

        let response = self.client
            .post(GITHUB_GRAPHQL_API_URL)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "github-graphql-client")
            .json(&serde_json::json!({
                "query": query
            }))
            .send().await?
            .json::<serde_json::Value>().await?;

        let repository = serde_json::from_value(response["data"]["repository"].clone())?;
        Ok(repository)
    }
}

struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn repository(
        &self,
        ctx: &Context<'_>,
        owner: String,
        name: String
    ) -> Result<Repository> {
        let github = ctx.data::<Arc<GithubClient>>()?;
        github.get_repository(&owner, &name).await
    }

    async fn repository_info(
        &self,
        ctx: &Context<'_>,
        owner: String,
        name: String
    ) -> Result<Repository> {
        let github = ctx.data::<Arc<GithubClient>>()?;
        let query = format!(
            r#"
            query {{
                repository(owner: "{}", name: "{}") {{
                    name
                    description
                    stargazerCount
                    forkCount
                    primaryLanguage {{
                        name
                        color
                    }}
                    updatedAt
                }}
            }}
            "#,
            owner,
            name
        );
        let response = github.client
            .post(GITHUB_GRAPHQL_API_URL)
            .header("Authorization", format!("Bearer {}", github.token))
            .header("User-Agent", "github-graphql-client")
            .json(&serde_json::json!({
                "query": query
            }))
            .send().await?
            .json::<serde_json::Value>().await?;
        let repository_info = serde_json::from_value(response["data"]["repository"].clone())?;
        Ok(repository_info)
    }
}

type ApiSchema = Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

async fn graphql_handler(
    schema: web::Data<ApiSchema>,
    req: HttpRequest,
    payload: web::Payload
) -> actix_web::Result<HttpResponse> {
    let request = async_graphql_actix_web::GraphQLRequest
        ::from_request(&req, &mut payload.into_inner()).await?
        .into_inner();
    let response = schema.execute(request).await;
    Ok(HttpResponse::Ok().json(response))
}

pub fn create_github_client(token: String) -> Arc<GithubClient> {
    Arc::new(GithubClient::new(token))
}

pub fn create_graphql_schema(github_client: Arc<GithubClient>) -> ApiSchema {
    Schema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .data(github_client)
        .finish()
}
