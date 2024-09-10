use axum::{middleware, Router};

use crate::middleware::mw_auth;

pub fn router() -> Router {
    Router::new()
        // Require authentication for all notes routes.
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth))
}

// TODO: Implement your routes here.
