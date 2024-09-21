use axum::{
    http::{Method, Uri},
    middleware as mw,
    response::{IntoResponse, Response},
    Json, Router,
};
use ctx::Ctx;
use log::log_request;
use response::action::RequestAction;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

// Re-export Error and Result
pub use self::error::{APIError, Result};

mod api;
mod ctx;
mod error;
mod log;
mod middleware;
mod response;
mod uuid;

#[tokio::main]
async fn main() {
    let api_router = Router::new()
        .nest("/api/v1/", api::router())
        .layer(mw::map_response(main_response_mapper))
        .layer(mw::from_fn(middleware::mw_auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, api_router).await.unwrap();
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_prefixed("req");

    // Get the eventual response error.
    let service_error = res.extensions().get::<APIError>();

    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let actions = res.extensions().get::<Vec<RequestAction>>();

    // If we have a client error, build a new response.

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "req_uuid": uuid.to_string(),
                "error": {
                    "type": client_error.as_ref(),
                },
                "actions": actions.unwrap_or(&Vec::new()),
            });

            println!("   ->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;

    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();

    error_response.unwrap_or(res)
}
