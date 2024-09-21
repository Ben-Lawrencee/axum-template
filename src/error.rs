use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

/// The API Result type.
pub type Result<T> = core::result::Result<T, APIError>;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum APIError {
    LoginFail,

    // TODO: Add more errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

impl APIError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            // Login
            Self::LoginFail => (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAILED),

            // Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response body.
        response.extensions_mut().insert(self);

        response
    }
}

impl std::fmt::Display for APIError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for APIError {}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAILED,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
