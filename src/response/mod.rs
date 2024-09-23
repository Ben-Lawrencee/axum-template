use action::RequestAction;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;

pub mod action;
pub mod method;

#[derive(Debug, Clone)]
pub struct APIResponse<T: Serialize> {
    status_code: StatusCode,
    data: T,
    actions: Vec<RequestAction>,
}

impl<T: Serialize> APIResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            data,
            actions: Vec::new(),
        }
    }

    pub fn with_actions(mut self, actions: Vec<RequestAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn with_action(mut self, action: RequestAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_status(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl<T: Serialize> IntoResponse for APIResponse<T> {
    fn into_response(self) -> Response<Body> {
        (
            self.status_code,
            Json(json!({
                "data": self.data,
                "actions": self.actions,
            })),
        )
            .into_response()
    }
}
