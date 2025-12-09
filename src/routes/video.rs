use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use crate::error::AppError;
use crate::tiktok::{self, types::VideoInfo};

#[derive(Template)]
#[template(path = "video.html")]
struct VideoTemplate {
    video: VideoInfo,
}

async fn get_video(Path(video_id): Path<String>) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Fetching video: {}", video_id);
    
    let video = tiktok::client::fetch_video(&video_id).await?;
    
    let template = VideoTemplate { video };
    Ok(Html(template.render().map_err(|_| AppError::Internal)?))
}

pub fn router() -> Router {
    Router::new()
        .route("/video/{video_id}", get(get_video))
}
