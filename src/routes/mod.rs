use axum::{
    Json,
    Router,
    routing::{delete, get, post, put},
};
use serde::Serialize;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    error::ErrorResponse,
    handlers,
    models::{
        CreateProjectRequest, CreateTechnologyRequest, CreateUserRequest,
        ListQueryParams, PaginatedResponse, PaginationMetadata, Project, ProjectWithRelations,
        Technology, UpdateProjectRequest, User, UserRole, UserWithRole,
    },
    state::AppState,
};

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::projects::list_projects,
        crate::handlers::projects::get_project,
        crate::handlers::projects::create_project,
        crate::handlers::projects::update_project,
        crate::handlers::projects::delete_project,
        crate::handlers::technologies::list_technologies,
        crate::handlers::technologies::create_technology,
        crate::handlers::users::list_users,
        crate::handlers::users::create_user,
    ),
    components(
        schemas(
            Project, CreateProjectRequest, UpdateProjectRequest, ProjectWithRelations,
            Technology, CreateTechnologyRequest,
            User, CreateUserRequest, UserRole, UserWithRole,
            PaginatedResponse<Project>, PaginationMetadata, ListQueryParams,
            ErrorResponse, HealthResponse
        )
    ),
    tags(
        (name = "projects", description = "Project management endpoints"),
        (name = "technologies", description = "Technology management endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Projects API",
        version = "0.1.0",
        description = "A modern REST API for managing code projects with technologies and users, built with Axum and Rust",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT"
        )
    )
)]
pub struct ApiDoc;

/// Health check response
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    /// Status message
    pub status: String,
}

/// Creates the application router with all routes configured
///
/// # Routes
/// - `GET /health` - Health check endpoint
/// - `GET /projects` - List all projects with advanced filtering and pagination
/// - `GET /projects/{id}` - Get a specific project with relations
/// - `POST /projects` - Create a new project
/// - `PUT /projects/{id}` - Update a project
/// - `DELETE /projects/{id}` - Delete a project
/// - `GET /technologies` - List all technologies
/// - `POST /technologies` - Create a new technology
/// - `GET /users` - List all users
/// - `POST /users` - Create a new user
/// - `GET /swagger-ui` - Swagger UI documentation
pub fn create_router(state: AppState) -> Router {
    // Create the API router
    let api_router = Router::new()
        // Health check
        .route("/health", get(health_check))
        // Projects routes
        .route("/projects", get(handlers::list_projects))
        .route("/projects", post(handlers::create_project))
        .route("/projects/{id}", get(handlers::get_project))
        .route("/projects/{id}", put(handlers::update_project))
        .route("/projects/{id}", delete(handlers::delete_project))
        // Technologies routes
        .route("/technologies", get(handlers::list_technologies))
        .route("/technologies", post(handlers::create_technology))
        // Users routes
        .route("/users", get(handlers::list_users))
        .route("/users", post(handlers::create_user))
        // Share state across all routes
        .with_state(state);

    // Merge with Swagger UI (which doesn't need state)
    api_router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

/// Health check endpoint
///
/// Returns a simple status message to verify the API is running
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "API is healthy", body = HealthResponse)
    )
)]
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;
    use crate::state::tests::new_test_db;

    #[tokio::test]
    async fn test_health_check() {
        let state = new_test_db().await;
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_project_endpoint() {
        let state = new_test_db().await;
        let app = create_router(state);

        let request_body = json!({
            "name": "Test API Project",
            "description": "Testing the API",
            "repository_url": "https://github.com/test/api",
            "language": "Rust"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/projects")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_swagger_ui_available() {
        let state = new_test_db().await;
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/swagger-ui/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Swagger UI should redirect or return content
        assert!(response.status() == StatusCode::OK || response.status() == StatusCode::MOVED_PERMANENTLY);
    }
}
