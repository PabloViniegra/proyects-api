use proyects_api::{routes, state::AppState, middleware::RateLimiter};
use std::net::SocketAddr;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use axum::{
    Extension,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "proyects_api=debug,tower_http=debug,axum=trace,sqlx=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:proyects.db?mode=rwc".to_string());

    tracing::info!("Connecting to database: {}", database_url);

    // Initialize application state with database connection
    let state = AppState::new(&database_url)
        .await
        .expect("Failed to initialize database");

    tracing::info!("Database initialized successfully");

    // Configure CORS with allowed origins from environment
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".to_string())
        .split(',')
        .filter_map(|origin| {
            let trimmed = origin.trim();
            match trimmed.parse::<HeaderValue>() {
                Ok(header) => {
                    tracing::info!("CORS: Allowing origin: {}", trimmed);
                    Some(header)
                }
                Err(e) => {
                    tracing::warn!("CORS: Invalid origin '{}': {}", trimmed, e);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    // Configure rate limiting: 100 requests per second with burst of 20
    // This prevents API abuse and protects against DoS attacks
    let rate_limit_per_second = std::env::var("RATE_LIMIT_PER_SECOND")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(100);
    
    let rate_limit_burst = std::env::var("RATE_LIMIT_BURST")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(20);

    let rate_limiter = RateLimiter::new(rate_limit_per_second, rate_limit_burst);

    tracing::info!(
        "Rate limiting configured: {} req/s, burst size: {}",
        rate_limit_per_second,
        rate_limit_burst
    );

    // Create router with routes and middleware
    let app = routes::create_router(state)
        .layer(Extension(rate_limiter))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer);

    // Configure server address
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or_else(|e| {
            tracing::warn!("Invalid PORT value: {}. Using default 3000", e);
            3000
        });

    let host = std::env::var("HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse::<std::net::IpAddr>()
        .unwrap_or_else(|e| {
            tracing::warn!("Invalid HOST value: {}. Using default 0.0.0.0", e);
            "0.0.0.0".parse().unwrap()
        });

    let addr = SocketAddr::from((host, port));

    tracing::info!("Starting server on {}", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server running at http://{}", addr);
    tracing::info!("Health check available at http://{}/health", addr);
    tracing::info!("Projects API available at http://{}/projects", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui/", addr);
    tracing::info!("OpenAPI spec available at http://{}/api-docs/openapi.json", addr);

    // Start server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
