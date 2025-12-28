use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::time::Duration;

/// Application state shared across handlers
///
/// Contains the database connection pool for SQLite
#[derive(Clone)]
pub struct AppState {
    /// SQLite connection pool
    pub db: SqlitePool,
}

impl AppState {
    /// Creates a new AppState instance with a database connection pool
    ///
    /// # Arguments
    ///
    /// * `database_url` - The SQLite database URL
    ///
    /// # Example
    ///
    /// ```no_run
    /// use proyects_api::state::AppState;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let state = AppState::new("sqlite:proyects.db?mode=rwc").await.unwrap();
    /// }
    /// ```
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&db)
            .await?;

        Ok(Self { db })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Create a unique test database for isolated testing
    /// 
    /// Each call creates a new in-memory SQLite database with a unique name
    /// to prevent test interference when running in parallel
    pub async fn new_test_db() -> AppState {
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::thread;
        
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        
        // Create a truly unique database name using thread ID, counter, and timestamp
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let thread_id = format!("{:?}", thread::current().id());
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let unique_name = format!("test_{}_{}_{}",
            thread_id.replace("ThreadId(", "").replace(")", ""),
            counter,
            timestamp
        );
        
        let db_url = format!("sqlite:file:{}?mode=memory&cache=shared", unique_name);
        
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_url)
            .await
            .unwrap();

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .unwrap();

        // Clear all seed data from tables to ensure clean test state
        // This removes data inserted by seed_test_data.sql migration
        sqlx::query("DELETE FROM project_users").execute(&db).await.unwrap();
        sqlx::query("DELETE FROM project_technologies").execute(&db).await.unwrap();
        sqlx::query("DELETE FROM projects").execute(&db).await.unwrap();
        sqlx::query("DELETE FROM users").execute(&db).await.unwrap();
        sqlx::query("DELETE FROM technologies").execute(&db).await.unwrap();

        AppState { db }
    }

    #[tokio::test]
    async fn test_app_state_creation() {
        let state = new_test_db().await;
        // If we got here, creation was successful
        assert!(sqlx::query("SELECT 1").fetch_one(&state.db).await.is_ok());
    }

    #[tokio::test]
    async fn test_app_state_with_migrations() {
        let state = new_test_db().await;

        // Verify the table was created by running a simple query
        let result = sqlx::query("SELECT COUNT(*) as count FROM projects")
            .fetch_one(&state.db)
            .await;

        assert!(result.is_ok());
    }
}
