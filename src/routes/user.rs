use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use crate::error::AppError;
use crate::tiktok::{self, types::UserInfo};

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate {
    user: UserInfo,
}

async fn get_user(Path(username): Path<String>) -> Result<impl IntoResponse, AppError> {
    // Remove @ if present
    let username = username.trim_start_matches('@');
    
    tracing::info!("Fetching user: {}", username);
    
    let user = tiktok::client::fetch_user(username).await?;
    
    let template = UserTemplate { user };
    Ok(Html(template.render().map_err(|_| AppError::Internal)?))
}

pub fn router() -> Router {
    Router::new()
        .route("/@{username}", get(get_user))
}
