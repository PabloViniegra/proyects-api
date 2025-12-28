//! # Projects API
//!
//! A modern REST API for managing code projects, built with Axum and Rust.
//!
//! ## Features
//!
//! - CRUD operations for projects
//! - Input validation
//! - Structured error handling
//! - Async/await with Tokio
//! - Type-safe routing with Axum
//! - SQLite database with SQLx
//! - OpenAPI/Swagger documentation
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use proyects_api::{routes, state::AppState};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize database connection
//!     let state = AppState::new("sqlite:proyects.db")
//!         .await
//!         .expect("Failed to initialize database");
//!
//!     // Create router with all routes
//!     let app = routes::create_router(state);
//!
//!     // Start server
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
//!         .await
//!         .unwrap();
//!
//!     axum::serve(listener, app).await.unwrap();
//! }
//! ```

pub mod error;
pub mod extractors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod state;

// Re-export commonly used types
pub use error::{AppError, Result};
pub use state::AppState;
