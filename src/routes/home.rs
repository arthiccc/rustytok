use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    query: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: Option<String>,
}

async fn home(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    // If there's a search query, redirect to appropriate page
    if let Some(ref q) = params.q {
        let q = q.trim();
        
        // Handle different input types
        if q.starts_with('@') {
            // Username
            return Redirect::to(&format!("/{}", q)).into_response();
        } else if q.starts_with('#') {
            // Hashtag
            return Redirect::to(&format!("/tag/{}", &q[1..])).into_response();
        } else if q.contains("tiktok.com") {
            // TikTok URL - parse and redirect
            if let Some(path) = parse_tiktok_url(q) {
                return Redirect::to(&path).into_response();
            }
        } else if q.chars().all(|c| c.is_ascii_digit()) {
            // Video ID
            return Redirect::to(&format!("/video/{}", q)).into_response();
        } else {
            // Assume username without @
            return Redirect::to(&format!("/@{}", q)).into_response();
        }
    }
    
    let template = HomeTemplate { query: params.q };
    Html(template.render().unwrap()).into_response()
}

fn parse_tiktok_url(url: &str) -> Option<String> {
    // Handle various TikTok URL formats
    // https://www.tiktok.com/@username
    // https://www.tiktok.com/@username/video/1234567890
    // https://vm.tiktok.com/XXXXX
    // https://www.tiktok.com/t/XXXXX
    
    if url.contains("/@") {
        // User or video URL
        if let Some(at_pos) = url.find("/@") {
            let path = &url[at_pos..];
            // Remove query params if any
            let path = path.split('?').next().unwrap_or(path);
            return Some(path.to_string());
        }
    } else if url.contains("/video/") {
        // Direct video URL
        if let Some(video_pos) = url.find("/video/") {
            let video_id = &url[video_pos + 7..];
            let video_id = video_id.split('?').next().unwrap_or(video_id);
            let video_id = video_id.split('/').next().unwrap_or(video_id);
            return Some(format!("/video/{}", video_id));
        }
    } else if url.contains("vm.tiktok.com") || url.contains("/t/") {
        // Short URL - we'll handle redirect on the server
        return Some(format!("/redirect?url={}", urlencoding::encode(url)));
    } else if url.contains("/tag/") || url.contains("/discover/") {
        // Tag URL
        if let Some(tag_pos) = url.find("/tag/") {
            let tag = &url[tag_pos + 5..];
            let tag = tag.split('?').next().unwrap_or(tag);
            return Some(format!("/tag/{}", tag));
        }
    }
    
    None
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(home))
}
