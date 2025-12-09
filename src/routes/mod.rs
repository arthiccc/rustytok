mod home;
mod user;
mod video;
mod tag;
mod proxy;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(home::router())
        .merge(user::router())
        .merge(video::router())
        .merge(tag::router())
        .merge(proxy::router())
}
