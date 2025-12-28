use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use validator::Validate;

use crate::{
    error::{AppError, ErrorResponse, Result},
    models::{CreateUserRequest, User},
    state::AppState,
};

/// List all users
///
/// # Endpoint
/// GET /users
///
/// # Returns
/// - `200 OK` - List of all users
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    responses(
        (status = 200, description = "List of all users", body = [User]),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[tracing::instrument(skip(state))]
pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY name ASC"
    )
    .fetch_all(&state.db)
    .await?;

    tracing::info!("Listed {} users", users.len());
    Ok(Json(users))
}

/// Create a new user
///
/// # Endpoint
/// POST /users
///
/// # Request Body
/// ```json
/// {
///   "name": "John Doe",
///   "email": "john@example.com"
/// }
/// ```
///
/// # Returns
/// - `201 Created` - Created user
/// - `400 Bad Request` - Validation error
/// - `409 Conflict` - User with this email already exists
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[tracing::instrument(skip(state))]
pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>)> {
    // Validate request
    request.validate()?;

    // Check if user with this email already exists
    let existing = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = ?"
    )
    .bind(&request.email)
    .fetch_optional(&state.db)
    .await?;

    if existing.is_some() {
        return Err(AppError::DuplicateResource(format!(
            "User with email '{}' already exists",
            request.email
        )));
    }

    // Create new user
    let user = User::new(request);

    // Insert into database
    sqlx::query(
        "INSERT INTO users (id, name, email, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(user.id.to_string())
    .bind(&user.name)
    .bind(&user.email)
    .bind(user.created_at)
    .execute(&state.db)
    .await?;

    tracing::info!("Created user: {}", user.id);
    Ok((StatusCode::CREATED, Json(user)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::tests::new_test_db;

    #[tokio::test]
    async fn test_create_and_list_user() {
        let state = new_test_db().await;

        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        // Create user
        let (status, Json(created)) = create_user(State(state.clone()), Json(request))
            .await
            .unwrap();

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(created.name, "John Doe");
        assert_eq!(created.email, "john@example.com");

        // List users
        let Json(users) = list_users(State(state)).await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "John Doe");
    }

    #[tokio::test]
    async fn test_duplicate_user_email() {
        let state = new_test_db().await;

        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        // Create first user
        let _ = create_user(State(state.clone()), Json(request.clone()))
            .await
            .unwrap();

        // Try to create duplicate
        let result = create_user(State(state), Json(request)).await;
        assert!(result.is_err());
    }
}
