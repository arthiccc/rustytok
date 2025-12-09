use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("TikTok content not found")]
    NotFound,
    
    #[error("Failed to fetch from TikTok: {0}")]
    FetchError(String),
    
    #[error("Failed to parse TikTok response")]
    ParseError,
    
    #[error("Invalid URL format")]
    InvalidUrl,
    
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::FetchError(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::ParseError => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::InvalidUrl => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Error - RustyTok</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <main class="error-page">
        <h1>😿 {}</h1>
        <p>{}</p>
        <a href="/" class="btn">← Back to Home</a>
    </main>
</body>
</html>"#,
            status.as_u16(),
            message
        );

        (status, Html(html)).into_response()
    }
}
