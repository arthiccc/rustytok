mod config;
mod error;
mod routes;
mod tiktok;

use axum::{
    Router,
    middleware,
    response::Response,
    http::header,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Security middleware that adds privacy-focused headers
async fn security_headers(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    
    // Strong CSP - blocks all connections to TikTok
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; media-src 'self'; frame-ancestors 'none'; form-action 'self'"
            .parse()
            .unwrap(),
    );
    
    // Additional security headers
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        "nosniff".parse().unwrap(),
    );
    headers.insert(
        header::X_FRAME_OPTIONS,
        "DENY".parse().unwrap(),
    );
    headers.insert(
        header::REFERRER_POLICY,
        "no-referrer".parse().unwrap(),
    );
    
    response
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "rustytok=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();
    let config = config::Config::from_env();
    
    tracing::info!("🦀 RustyTok starting on port {}", config.port);

    // Build router
    let app = Router::new()
        .merge(routes::router())
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware::from_fn(security_headers));

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    tracing::info!("🚀 Server running at http://localhost:{}", config.port);
    
    axum::serve(listener, app).await.unwrap();
}
