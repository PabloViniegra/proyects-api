use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, sqlite::SqliteRow, Row};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use super::technology::Technology;
use super::user::UserWithRole;

/// Represents a code project in the system
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Project {
    /// Unique identifier for the project
    pub id: Uuid,
    /// Name of the project
    pub name: String,
    /// Detailed description of the project
    pub description: String,
    /// URL to the source code repository
    pub repository_url: String,
    /// Programming language used in the project
    pub language: String,
    /// Project rating (0.0 - 5.0)
    pub rating: Option<f64>,
    /// Timestamp when the project was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the project was last updated
    pub updated_at: DateTime<Utc>,
}

// Custom FromRow implementation to handle UUID as TEXT in SQLite
impl FromRow<'_, SqliteRow> for Project {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(Project {
            id,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            repository_url: row.try_get("repository_url")?,
            language: row.try_get("language")?,
            rating: row.try_get("rating")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl Project {
    /// Creates a new Project from a CreateProjectRequest
    ///
    /// # Example
    ///
    /// ```
    /// use proyects_api::models::CreateProjectRequest;
    /// use proyects_api::models::Project;
    ///
    /// let request = CreateProjectRequest {
    ///     name: "My Project".to_string(),
    ///     description: "A sample project".to_string(),
    ///     repository_url: "https://github.com/user/repo".to_string(),
    ///     language: "Rust".to_string(),
    ///     rating: Some(4.5),
    ///     technology_ids: None,
    ///     user_ids: None,
    /// };
    ///
    /// let project = Project::new(request);
    /// assert_eq!(project.name, "My Project");
    /// ```
    pub fn new(request: CreateProjectRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: request.name,
            description: request.description,
            repository_url: request.repository_url,
            language: request.language,
            rating: request.rating,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the project with new data
    ///
    /// # Arguments
    ///
    /// * `update` - The update request containing fields to update
    pub fn update(&mut self, update: UpdateProjectRequest) {
        if let Some(name) = update.name {
            self.name = name;
        }
        if let Some(description) = update.description {
            self.description = description;
        }
        if let Some(repository_url) = update.repository_url {
            self.repository_url = repository_url;
        }
        if let Some(language) = update.language {
            self.language = language;
        }
        if update.rating.is_some() {
            self.rating = update.rating;
        }
        self.updated_at = Utc::now();
    }
}

/// Request payload for creating a new project
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateProjectRequest {
    /// Name of the project
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    /// Description of the project
    #[validate(length(
        min = 1,
        max = 2000,
        message = "Description must be between 1 and 2000 characters"
    ))]
    pub description: String,

    /// Repository URL
    #[validate(url(message = "Repository URL must be a valid URL"))]
    pub repository_url: String,

    /// Programming language
    #[validate(length(
        min = 1,
        max = 100,
        message = "Language must be between 1 and 100 characters"
    ))]
    pub language: String,

    /// Optional rating (0.0 - 5.0)
    #[validate(range(min = 0.0, max = 5.0, message = "Rating must be between 0.0 and 5.0"))]
    pub rating: Option<f64>,

    /// Optional technology IDs to associate with the project
    pub technology_ids: Option<Vec<Uuid>>,

    /// Optional user IDs to associate with the project
    pub user_ids: Option<Vec<Uuid>>,
}

/// Request payload for updating an existing project
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateProjectRequest {
    /// Optional new name for the project
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    /// Optional new description
    #[validate(length(
        min = 1,
        max = 2000,
        message = "Description must be between 1 and 2000 characters"
    ))]
    pub description: Option<String>,

    /// Optional new repository URL
    #[validate(url(message = "Repository URL must be a valid URL"))]
    pub repository_url: Option<String>,

    /// Optional new language
    #[validate(length(
        min = 1,
        max = 100,
        message = "Language must be between 1 and 100 characters"
    ))]
    pub language: Option<String>,

    /// Optional new rating (0.0 - 5.0)
    #[validate(range(min = 0.0, max = 5.0, message = "Rating must be between 0.0 and 5.0"))]
    pub rating: Option<f64>,

    /// Optional technology IDs to replace existing associations
    pub technology_ids: Option<Vec<Uuid>>,

    /// Optional user IDs to replace existing associations
    pub user_ids: Option<Vec<Uuid>>,
}

/// Project with embedded related data
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProjectWithRelations {
    /// Project base information
    #[serde(flatten)]
    pub project: Project,
    /// Associated technologies
    pub technologies: Vec<Technology>,
    /// Associated users with their roles
    pub users: Vec<UserWithRole>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project() {
        let request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: "A test project".to_string(),
            repository_url: "https://github.com/test/repo".to_string(),
            language: "Rust".to_string(),
            rating: Some(4.5),
            technology_ids: None,
            user_ids: None,
        };

        let project = Project::new(request.clone());
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.description, "A test project");
        assert_eq!(project.repository_url, "https://github.com/test/repo");
        assert_eq!(project.language, "Rust");
        assert_eq!(project.rating, Some(4.5));
    }

    #[test]
    fn test_update_project() {
        let create_request = CreateProjectRequest {
            name: "Original".to_string(),
            description: "Original description".to_string(),
            repository_url: "https://github.com/original/repo".to_string(),
            language: "Rust".to_string(),
            rating: None,
            technology_ids: None,
            user_ids: None,
        };

        let mut project = Project::new(create_request);
        let original_created = project.created_at;

        let update_request = UpdateProjectRequest {
            name: Some("Updated".to_string()),
            description: None,
            repository_url: None,
            language: Some("Python".to_string()),
            rating: Some(3.5),
            technology_ids: None,
            user_ids: None,
        };

        project.update(update_request);

        assert_eq!(project.name, "Updated");
        assert_eq!(project.description, "Original description");
        assert_eq!(project.language, "Python");
        assert_eq!(project.rating, Some(3.5));
        assert_eq!(project.created_at, original_created);
        assert!(project.updated_at > original_created);
    }
}
