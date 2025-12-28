use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, sqlite::SqliteRow, Row};
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Represents a user in the system
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// Name of the user
    pub name: String,
    /// Email address (unique)
    pub email: String,
    /// Timestamp when the user was created
    pub created_at: DateTime<Utc>,
}

// Custom FromRow implementation to handle UUID as TEXT in SQLite
impl FromRow<'_, SqliteRow> for User {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(User {
            id,
            name: row.try_get("name")?,
            email: row.try_get("email")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl User {
    /// Creates a new User from a CreateUserRequest
    pub fn new(request: CreateUserRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: request.name,
            email: request.email,
            created_at: Utc::now(),
        }
    }
}

/// Request payload for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    /// Name of the user
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    /// Email address (must be unique)
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
}

/// User role in a project
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Owner,
    Contributor,
    Viewer,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Owner => "owner",
            UserRole::Contributor => "contributor",
            UserRole::Viewer => "viewer",
        }
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owner" => Ok(UserRole::Owner),
            "contributor" => Ok(UserRole::Contributor),
            "viewer" => Ok(UserRole::Viewer),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

/// User with role in a project context
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserWithRole {
    /// User information
    #[serde(flatten)]
    pub user: User,
    /// Role in the project
    pub role: UserRole,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        let user = User::new(request.clone());
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }

    #[test]
    fn test_validate_user_email() {
        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "invalid-email".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_user_role_conversion() {
        assert_eq!(UserRole::from_str("owner").unwrap(), UserRole::Owner);
        assert_eq!(UserRole::from_str("contributor").unwrap(), UserRole::Contributor);
        assert_eq!(UserRole::from_str("viewer").unwrap(), UserRole::Viewer);
        assert!(UserRole::from_str("invalid").is_err());

        assert_eq!(UserRole::Owner.as_str(), "owner");
        assert_eq!(UserRole::Contributor.as_str(), "contributor");
        assert_eq!(UserRole::Viewer.as_str(), "viewer");
    }
}
