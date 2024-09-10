use axum::Router;

// Auth
pub mod login;

// Services
pub mod notes;

pub const AUTH_TOKEN: &str = "auth_token";

pub fn router() -> Router {
    Router::new()
        .nest("/login", login::router())
        .nest("/notes", notes::router())
}
