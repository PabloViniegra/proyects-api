use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationMetadata {
    /// Current page number
    pub page: u32,
    /// Number of items per page
    pub page_size: u32,
    /// Total number of items
    pub total_items: i64,
    /// Total number of pages
    pub total_pages: u32,
}

impl PaginationMetadata {
    pub fn new(page: u32, page_size: u32, total_items: i64) -> Self {
        let total_pages = ((total_items as f64) / (page_size as f64)).ceil() as u32;
        Self {
            page,
            page_size,
            total_items,
            total_pages: total_pages.max(1),
        }
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    /// The data items
    pub data: Vec<T>,
    /// Pagination metadata
    pub pagination: PaginationMetadata,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, page_size: u32, total_items: i64) -> Self {
        Self {
            data,
            pagination: PaginationMetadata::new(page, page_size, total_items),
        }
    }
}

/// Query parameters for list endpoints
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ListQueryParams {
    /// Search text in name and description
    pub search: Option<String>,
    /// Filter by technology name
    #[serde(alias = "tech")]
    pub technology: Option<String>,
    /// Filter by user ID
    pub user_id: Option<String>,
    /// Minimum rating filter
    pub min_rating: Option<f64>,
    /// Maximum rating filter
    pub max_rating: Option<f64>,
    /// Filter by language
    pub language: Option<String>,
    /// Field to sort by (name, created_at, updated_at, rating)
    pub sort: Option<String>,
    /// Sort order (asc, desc)
    pub order: Option<String>,
    /// Page number (default: 1)
    pub page: Option<u32>,
    /// Items per page (default: 10, max: 100)
    pub page_size: Option<u32>,
}

impl ListQueryParams {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> u32 {
        (self.page() - 1) * self.page_size()
    }

    pub fn sort_field(&self) -> &str {
        match self.sort.as_deref() {
            Some("name") => "name",
            Some("created_at") => "created_at",
            Some("updated_at") => "updated_at",
            Some("rating") => "rating",
            _ => "created_at",
        }
    }

    pub fn sort_order(&self) -> &str {
        match self.order.as_deref() {
            Some("asc") => "ASC",
            Some("desc") => "DESC",
            _ => "DESC",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_metadata() {
        let meta = PaginationMetadata::new(1, 10, 45);
        assert_eq!(meta.page, 1);
        assert_eq!(meta.page_size, 10);
        assert_eq!(meta.total_items, 45);
        assert_eq!(meta.total_pages, 5);
    }

    #[test]
    fn test_list_query_params_defaults() {
        let params = ListQueryParams {
            search: None,
            technology: None,
            user_id: None,
            min_rating: None,
            max_rating: None,
            language: None,
            sort: None,
            order: None,
            page: None,
            page_size: None,
        };

        assert_eq!(params.page(), 1);
        assert_eq!(params.page_size(), 10);
        assert_eq!(params.offset(), 0);
        assert_eq!(params.sort_field(), "created_at");
        assert_eq!(params.sort_order(), "DESC");
    }

    #[test]
    fn test_list_query_params_custom() {
        let params = ListQueryParams {
            search: None,
            technology: None,
            user_id: None,
            min_rating: None,
            max_rating: None,
            language: None,
            sort: Some("name".to_string()),
            order: Some("asc".to_string()),
            page: Some(2),
            page_size: Some(20),
        };

        assert_eq!(params.page(), 2);
        assert_eq!(params.page_size(), 20);
        assert_eq!(params.offset(), 20);
        assert_eq!(params.sort_field(), "name");
        assert_eq!(params.sort_order(), "ASC");
    }

    #[test]
    fn test_page_size_limits() {
        let params = ListQueryParams {
            search: None,
            technology: None,
            user_id: None,
            min_rating: None,
            max_rating: None,
            language: None,
            sort: None,
            order: None,
            page: None,
            page_size: Some(200),
        };

        assert_eq!(params.page_size(), 100);
    }
}
