use juniper::GraphQLObject;

// Define the blog text type
#[derive(GraphQLObject)]
pub struct BlogText {
    id: String,
    title: String,
    content: String,
}

// Define the blog stats type
#[derive(GraphQLObject)]
pub struct BlogStats {
    total_posts: i32,
    total_comments: i32,
    avg_likes: f64,
}
