use uuid::Uuid;

/// Custom UUID wrapper for validated path parameters
///
/// This module provides utilities for UUID validation in path parameters.
/// In Axum 0.8+, the built-in `Path<Uuid>` extractor already provides
/// good error messages, so this is primarily for future customization needs.
///
/// # Example
///
/// ```rust,ignore
/// use axum::{Router, routing::get, Json, extract::Path};
/// use uuid::Uuid;
///
/// async fn get_project(Path(id): Path<Uuid>) -> Json<String> {
///     // id is already validated by Axum
///     Json(format!("Project ID: {}", id))
/// }
///
/// let app = Router::new().route("/projects/:id", get(get_project));
/// ```
pub struct ValidatedUuid(pub Uuid);

impl ValidatedUuid {
    /// Create a new ValidatedUuid from a Uuid
    pub fn new(uuid: Uuid) -> Self {
        ValidatedUuid(uuid)
    }

    /// Get the inner UUID value
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ValidatedUuid {
    fn from(uuid: Uuid) -> Self {
        ValidatedUuid(uuid)
    }
}

impl From<ValidatedUuid> for Uuid {
    fn from(validated: ValidatedUuid) -> Self {
        validated.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_validation() {
        // Valid UUIDs
        let valid = "550e8400-e29b-41d4-a716-446655440000";
        assert!(Uuid::parse_str(valid).is_ok());

        // Invalid UUIDs
        let invalid = "not-a-uuid";
        assert!(Uuid::parse_str(invalid).is_err());

        let invalid_format = "550e8400-e29b-41d4-a716";
        assert!(Uuid::parse_str(invalid_format).is_err());
    }
}
