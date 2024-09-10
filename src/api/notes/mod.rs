use axum::{middleware, Router};

use crate::middleware::mw_auth;

pub fn router() -> Router {
    Router::new()
    .route_layer(middleware::from_fn(mw_auth::mw_require_auth))
}