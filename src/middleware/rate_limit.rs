use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Simple rate limiter based on IP address
/// Tracks requests per IP and enforces limits
#[derive(Clone)]
pub struct RateLimiter {
    /// Requests per second allowed per IP
    #[allow(dead_code)]
    per_second: u64,
    /// Burst size for short spikes
    burst_size: u32,
    /// State tracking requests per IP
    state: Arc<Mutex<RateLimiterState>>,
}

struct RateLimiterState {
    /// Map of IP -> request tracking
    requests: HashMap<IpAddr, RequestTracker>,
    /// Last cleanup time
    last_cleanup: Instant,
}

struct RequestTracker {
    /// Timestamps of recent requests
    requests: Vec<Instant>,
    /// Last request time
    last_request: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(per_second: u64, burst_size: u32) -> Self {
        Self {
            per_second,
            burst_size,
            state: Arc::new(Mutex::new(RateLimiterState {
                requests: HashMap::new(),
                last_cleanup: Instant::now(),
            })),
        }
    }

    /// Check if a request from this IP is allowed
    pub fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let mut state = self.state.lock().unwrap();

        // Clean up old entries every 60 seconds
        if state.last_cleanup.elapsed() > Duration::from_secs(60) {
            state.requests.retain(|_, tracker| {
                tracker.last_request.elapsed() < Duration::from_secs(60)
            });
            state.last_cleanup = Instant::now();
        }

        let now = Instant::now();
        let one_second_ago = now - Duration::from_secs(1);

        // Get or create tracker for this IP
        let tracker = state.requests.entry(ip).or_insert_with(|| RequestTracker {
            requests: Vec::new(),
            last_request: now,
        });

        // Remove requests older than 1 second
        tracker.requests.retain(|&time| time > one_second_ago);

        // Check if we're within limits
        if tracker.requests.len() < self.burst_size as usize {
            tracker.requests.push(now);
            tracker.last_request = now;
            true
        } else {
            false
        }
    }
}

/// Middleware function for rate limiting
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Response {
    // Extract IP from request
    let ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
        .or_else(|| {
            request
                .extensions()
                .get::<std::net::SocketAddr>()
                .map(|addr| addr.ip())
        })
        .unwrap_or(IpAddr::from([127, 0, 0, 1]));

    // Get rate limiter from extensions
    let rate_limiter = request
        .extensions()
        .get::<RateLimiter>()
        .expect("RateLimiter not found in extensions");

    if rate_limiter.check_rate_limit(ip) {
        // Request allowed
        next.run(request).await
    } else {
        // Rate limit exceeded
        (
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded. Please try again later.",
        )
            .into_response()
    }
}
