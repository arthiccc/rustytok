use once_cell::sync::Lazy;
use reqwest::Client;

use crate::error::AppError;
use super::parser;
use super::types::{UserInfo, VideoInfo, TagInfo};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client")
});

pub fn get_http_client() -> &'static Client {
    &HTTP_CLIENT
}

/// Fetch user profile and videos
pub async fn fetch_user(username: &str) -> Result<UserInfo, AppError> {
    let url = format!("https://www.tiktok.com/@{}", username);
    
    let response = HTTP_CLIENT
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| AppError::FetchError(e.to_string()))?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(AppError::NotFound);
    }
    
    if !response.status().is_success() {
        return Err(AppError::FetchError(format!("Status: {}", response.status())));
    }
    
    let html = response.text().await.map_err(|e| AppError::FetchError(e.to_string()))?;
    
    parser::parse_user_page(&html, username)
}

/// Fetch single video
pub async fn fetch_video(video_id: &str) -> Result<VideoInfo, AppError> {
    // Try to fetch the video page directly
    let url = format!("https://www.tiktok.com/video/{}", video_id);
    
    let response = HTTP_CLIENT
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| AppError::FetchError(e.to_string()))?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(AppError::NotFound);
    }
    
    if !response.status().is_success() {
        return Err(AppError::FetchError(format!("Status: {}", response.status())));
    }
    
    let html = response.text().await.map_err(|e| AppError::FetchError(e.to_string()))?;
    
    parser::parse_video_page(&html, video_id)
}

/// Fetch tag/hashtag videos
pub async fn fetch_tag(tag_name: &str) -> Result<TagInfo, AppError> {
    let url = format!("https://www.tiktok.com/tag/{}", tag_name);
    
    let response = HTTP_CLIENT
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| AppError::FetchError(e.to_string()))?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(AppError::NotFound);
    }
    
    if !response.status().is_success() {
        return Err(AppError::FetchError(format!("Status: {}", response.status())));
    }
    
    let html = response.text().await.map_err(|e| AppError::FetchError(e.to_string()))?;
    
    parser::parse_tag_page(&html, tag_name)
}
