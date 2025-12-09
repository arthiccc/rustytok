use axum::{
    body::Body,
    extract::Query,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;

use crate::error::AppError;
use crate::tiktok::client::get_http_client;

#[derive(Deserialize)]
pub struct ProxyQuery {
    url: String,
}

/// Proxy media (video/images) through our server to prevent TikTok tracking
async fn proxy_media(Query(params): Query<ProxyQuery>) -> Result<impl IntoResponse, AppError> {
    let url = urlencoding::decode(&params.url)
        .map_err(|_| AppError::InvalidUrl)?
        .to_string();
    
    // Only allow TikTok CDN URLs
    if !is_allowed_url(&url) {
        return Err(AppError::InvalidUrl);
    }
    
    tracing::debug!("Proxying media: {}", url);
    
    let client = get_http_client();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::FetchError(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(AppError::NotFound);
    }
    
    // Get content type
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    
    // Stream the response body
    let stream = response.bytes_stream();
    let body = Body::from_stream(stream);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=86400")
        .body(body)
        .unwrap())
}

fn is_allowed_url(url: &str) -> bool {
    // Only allow TikTok CDN domains
    let allowed_domains = [
        "tiktokcdn.com",
        "tiktokcdn-us.com",
        "tiktokv.com",
        "muscdn.com",
        "byteoversea.com",
        "ibytedtos.com",
        "tiktokcdn-in.com",
    ];
    
    allowed_domains.iter().any(|domain| url.contains(domain))
}

pub fn router() -> Router {
    Router::new()
        .route("/proxy", get(proxy_media))
}
