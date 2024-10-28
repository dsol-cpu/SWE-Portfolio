use actix_web::{ web, HttpResponse, Result };
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use serde_with::{ serde_as, TimestampSeconds };
use crate::lib::supabase::{ SupabaseError, SupabaseClient };

// Structs for different data types
#[serde_as]
#[derive(Serialize, Deserialize)]
struct Project {
    id: i32,
    title: String,
    description: String,
    technologies: Vec<String>,
    github_url: Option<String>,
    live_url: Option<String>,
    #[serde_as(as = "TimestampSeconds<i64>")]
    created_at: DateTime<Utc>,
}
impl Project {
    const TABLE: &'static str = "projects";

    pub async fn find_all(client: &SupabaseClient) -> Result<Vec<Project>, SupabaseError> {
        client.select::<Project>(Self::TABLE, None).await
    }

    pub async fn find_by_id(
        client: &SupabaseClient,
        id: i32
    ) -> Result<Option<Project>, SupabaseError> {
        client.from::<Project>(Self::TABLE).eq("id", &id.to_string()).execute_single().await
    }

    pub async fn search(
        client: &SupabaseClient,
        query: &str,
        technologies: Option<Vec<String>>,
        limit: Option<i32>,
        offset: Option<i32>
    ) -> Result<Vec<Project>, SupabaseError> {
        let mut builder = client
            .from::<Project>(Self::TABLE)
            .like("title", &format!("%{}%", query));

        if let Some(techs) = technologies {
            builder = builder.in_array("technologies", techs);
        }

        if let Some(limit) = limit {
            builder = builder.limit(limit);
        }

        if let Some(offset) = offset {
            builder = builder.offset(offset);
        }

        builder.execute().await
    }

    pub async fn create(
        client: &SupabaseClient,
        project: &Project
    ) -> Result<Project, SupabaseError> {
        client.insert(Self::TABLE, project).await
    }

    pub async fn update(
        client: &SupabaseClient,
        id: i32,
        project: &Project
    ) -> Result<Project, SupabaseError> {
        client.update(Self::TABLE, id, project).await
    }

    pub async fn delete(client: &SupabaseClient, id: i32) -> Result<(), SupabaseError> {
        client.delete(Self::TABLE, id).await
    }
}

pub async fn get_projects(
    client: web::Data<SupabaseClient>
) -> Result<HttpResponse, SupabaseError> {
    let projects: Vec<Project> = Project::find_all(&client).await?;
    Ok(HttpResponse::Ok().json(projects))
}

pub async fn get_project(
    client: web::Data<SupabaseClient>,
    path: web::Path<i32>
) -> Result<HttpResponse, SupabaseError> {
    let project_id: i32 = path.into_inner();
    match Project::find_by_id(&client, project_id).await? {
        Some(project) => Ok(HttpResponse::Ok().json(project)),
        None => Err(SupabaseError::NotFound),
    }
}
