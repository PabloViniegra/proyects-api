use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, sqlite::SqliteRow, Row};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Represents a technology in the system
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Technology {
    /// Unique identifier for the technology
    pub id: Uuid,
    /// Name of the technology (unique)
    pub name: String,
    /// Optional description of the technology
    pub description: Option<String>,
    /// Timestamp when the technology was created
    pub created_at: DateTime<Utc>,
}

// Custom FromRow implementation to handle UUID as TEXT in SQLite
impl FromRow<'_, SqliteRow> for Technology {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(Technology {
            id,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl Technology {
    /// Creates a new Technology from a CreateTechnologyRequest
    pub fn new(request: CreateTechnologyRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: request.name,
            description: request.description,
            created_at: Utc::now(),
        }
    }
}

/// Request payload for creating a new technology
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateTechnologyRequest {
    /// Name of the technology (must be unique)
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    /// Optional description
    #[validate(length(
        max = 500,
        message = "Description must be at most 500 characters"
    ))]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_technology() {
        let request = CreateTechnologyRequest {
            name: "Rust".to_string(),
            description: Some("A systems programming language".to_string()),
        };

        let tech = Technology::new(request.clone());
        assert_eq!(tech.name, "Rust");
        assert_eq!(tech.description, Some("A systems programming language".to_string()));
    }

    #[test]
    fn test_validate_technology_name() {
        let request = CreateTechnologyRequest {
            name: "".to_string(),
            description: None,
        };

        assert!(request.validate().is_err());
    }
}
