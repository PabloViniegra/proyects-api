use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use validator::Validate;

use crate::{
    error::{AppError, ErrorResponse, Result},
    models::{CreateTechnologyRequest, Technology},
    state::AppState,
};

/// List all technologies
///
/// # Endpoint
/// GET /technologies
///
/// # Returns
/// - `200 OK` - List of all technologies
#[utoipa::path(
    get,
    path = "/technologies",
    tag = "technologies",
    responses(
        (status = 200, description = "List of all technologies", body = [Technology]),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[tracing::instrument(skip(state))]
pub async fn list_technologies(State(state): State<AppState>) -> Result<Json<Vec<Technology>>> {
    let technologies = sqlx::query_as::<_, Technology>(
        "SELECT * FROM technologies ORDER BY name ASC"
    )
    .fetch_all(&state.db)
    .await?;

    tracing::info!("Listed {} technologies", technologies.len());
    Ok(Json(technologies))
}

/// Create a new technology
///
/// # Endpoint
/// POST /technologies
///
/// # Request Body
/// ```json
/// {
///   "name": "Rust",
///   "description": "A systems programming language"
/// }
/// ```
///
/// # Returns
/// - `201 Created` - Created technology
/// - `400 Bad Request` - Validation error
/// - `409 Conflict` - Technology with this name already exists
#[utoipa::path(
    post,
    path = "/technologies",
    tag = "technologies",
    request_body = CreateTechnologyRequest,
    responses(
        (status = 201, description = "Technology created successfully", body = Technology),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 409, description = "Technology already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[tracing::instrument(skip(state))]
pub async fn create_technology(
    State(state): State<AppState>,
    Json(request): Json<CreateTechnologyRequest>,
) -> Result<(StatusCode, Json<Technology>)> {
    // Validate request
    request.validate()?;

    // Check if technology with this name already exists
    let existing = sqlx::query_as::<_, Technology>(
        "SELECT * FROM technologies WHERE name = ?"
    )
    .bind(&request.name)
    .fetch_optional(&state.db)
    .await?;

    if existing.is_some() {
        return Err(AppError::DuplicateResource(format!(
            "Technology with name '{}' already exists",
            request.name
        )));
    }

    // Create new technology
    let technology = Technology::new(request);

    // Insert into database
    sqlx::query(
        "INSERT INTO technologies (id, name, description, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(technology.id.to_string())
    .bind(&technology.name)
    .bind(&technology.description)
    .bind(technology.created_at)
    .execute(&state.db)
    .await?;

    tracing::info!("Created technology: {}", technology.id);
    Ok((StatusCode::CREATED, Json(technology)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::tests::new_test_db;

    #[tokio::test]
    async fn test_create_and_list_technology() {
        let state = new_test_db().await;

        let request = CreateTechnologyRequest {
            name: "Rust".to_string(),
            description: Some("A systems programming language".to_string()),
        };

        // Create technology
        let (status, Json(created)) = create_technology(State(state.clone()), Json(request))
            .await
            .unwrap();

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(created.name, "Rust");

        // List technologies
        let Json(technologies) = list_technologies(State(state)).await.unwrap();
        assert_eq!(technologies.len(), 1);
        assert_eq!(technologies[0].name, "Rust");
    }

    #[tokio::test]
    async fn test_duplicate_technology_name() {
        let state = new_test_db().await;

        let request = CreateTechnologyRequest {
            name: "Rust".to_string(),
            description: None,
        };

        // Create first technology
        let _ = create_technology(State(state.clone()), Json(request.clone()))
            .await
            .unwrap();

        // Try to create duplicate
        let result = create_technology(State(state), Json(request)).await;
        assert!(result.is_err());
    }
}
