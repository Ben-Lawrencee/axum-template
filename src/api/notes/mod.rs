use axum::{extract::NestedPath, http::Method, middleware, routing::get, Router};

use crate::{
    middleware::mw_auth,
    response::{action::RequestAction, APIResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_notes))
        // Require authentication for all notes routes.
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth))
}

// TODO: Implement your routes here.

async fn get_notes(path: NestedPath) -> APIResponse<Vec<u16>> {
    APIResponse::new(vec![]).with_action(RequestAction::new(
        "Get notes",
        "Get all the users notes.",
        format!("{}/", path.as_str()),
        Method::GET,
    ))
}
