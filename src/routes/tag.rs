use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use crate::error::AppError;
use crate::tiktok::{self, types::TagInfo};

#[derive(Template)]
#[template(path = "tag.html")]
struct TagTemplate {
    tag: TagInfo,
}

async fn get_tag(Path(tag_name): Path<String>) -> Result<impl IntoResponse, AppError> {
    // Remove # if present
    let tag_name = tag_name.trim_start_matches('#');
    
    tracing::info!("Fetching tag: {}", tag_name);
    
    let tag = tiktok::client::fetch_tag(tag_name).await?;
    
    let template = TagTemplate { tag };
    Ok(Html(template.render().map_err(|_| AppError::Internal)?))
}

pub fn router() -> Router {
    Router::new()
        .route("/tag/{tag_name}", get(get_tag))
}
