use axum::{http::Method, middleware, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{
    middleware::mw_auth,
    response::{action::RequestAction, method::HTTPMethod},
    uuid::Uuid,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_notes))
        // Require authentication for all notes routes.
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth))
}

// TODO: Implement your routes here.

async fn get_notes() -> Json<Value> {
    Json(json!({
        "data": [],
        "actions": [
            RequestAction::new(
                Uuid::new_prefixed("act"),
                "Create a note".to_string(),
                "Create a new note in one of your collections.".to_string(),
                "/notes".to_string(),
                HTTPMethod::new(Method::POST)
            )
        ]
    }))
}
