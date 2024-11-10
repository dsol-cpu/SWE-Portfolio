use actix_web::{ get, web, HttpResponse, Responder };
use reqwest::Client;
use serde::{ Deserialize, Serialize };

// Response types
#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPost {
    id: String,
    title: String,
    author: String,
    content: String,
    published_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlogResponse {
    success: bool,
    posts: Vec<BlogPost>,
    message: Option<String>,
}

// Error type for the service
#[derive(Debug, Serialize)]
struct ServiceError {
    message: String,
}

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        ServiceError {
            message: error.to_string(),
        }
    }
}

#[get("/restapi/linkedin-blog")]
pub async fn get_linked_blog() -> impl Responder {
    let client = Client::new();

    // Configure this with your actual LinkedIn API endpoint
    match std::env::var("LINKEDIN_API_URL") {
        Ok(linkedin_api_url) => {
            match fetch_blog_posts(&client, &linkedin_api_url).await {
                Ok(posts) =>
                    HttpResponse::Ok().json(BlogResponse {
                        success: true,
                        posts,
                        message: None,
                    }),
                Err(e) => {
                    // Handle the error here...
                    eprintln!("Failed to fetch blog posts: {:?}", e);
                    HttpResponse::InternalServerError().body("Failed to fetch blog posts")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve LinkedIn API URL"),
    }
}

// GraphQL endpoint
#[get("/graphql/linkedin-blog")]
pub async fn get_linked_blog_graphql() -> impl Responder {
    let client = Client::new();

    // Configure this with your actual LinkedIn GraphQL endpoint
    let graphql_endpoint = "https://api.linkedin.com/v2/graphql";

    let query = GraphQLQuery {
        query: r#"
            query GetBlogPosts {
                posts {
                    id
                    title
                    author
                    content
                    publishedDate
                }
            }
        "#.to_string(),
        variables: None,
    };

    match fetch_blog_posts_graphql(&client, graphql_endpoint, query).await {
        Ok(posts) =>
            HttpResponse::Ok().json(BlogResponse {
                success: true,
                posts,
                message: None,
            }),
        Err(e) =>
            HttpResponse::InternalServerError().json(BlogResponse {
                success: false,
                posts: vec![],
                message: Some(e.message),
            }),
    }
}

// Helper function to fetch blog posts via REST
async fn fetch_blog_posts(client: &Client, url: &str) -> Result<Vec<BlogPost>, ServiceError> {
    let response = client
        .get(url)
        // Add your authentication headers here
        .header("Authorization", "Bearer YOUR_ACCESS_TOKEN")
        .send().await?;

    if response.status().is_success() {
        let posts = response.json::<Vec<BlogPost>>().await?;
        Ok(posts)
    } else {
        Err(ServiceError {
            message: format!("Failed to fetch blog posts: {}", response.status()),
        })
    }
}

// Helper function to fetch blog posts via GraphQL
async fn fetch_blog_posts_graphql(
    client: &Client,
    url: &str,
    query: GraphQLQuery
) -> Result<Vec<BlogPost>, ServiceError> {
    let response = client
        .post(url)
        .header("Authorization", "Bearer YOUR_ACCESS_TOKEN")
        .json(&query)
        .send().await?;

    if response.status().is_success() {
        let data: serde_json::Value = response.json().await?;

        // Parse the GraphQL response - adjust this based on your actual response structure
        match data.get("data").and_then(|d| d.get("posts")) {
            Some(posts) => {
                let posts = serde_json::from_value(posts.clone()).map_err(|e| ServiceError {
                    message: format!("Failed to parse posts: {}", e),
                })?;
                Ok(posts)
            }
            None =>
                Err(ServiceError {
                    message: "No posts found in response".to_string(),
                }),
        }
    } else {
        Err(ServiceError {
            message: format!("GraphQL query failed: {}", response.status()),
        })
    }
}

// Configuration function to register the services
pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_linked_blog).service(get_linked_blog_graphql);
}
