use axum::http::Method;
use serde::{Deserialize, Serialize};

use super::method::HTTPMethod;
use crate::uuid::{Prefixed, Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestAction {
    pub uuid: Uuid<Prefixed>,

    pub title: String,
    pub description: String,

    pub url: String,
    pub method: HTTPMethod,
}

impl RequestAction {
    pub fn builder() -> ResponseActionBuilder {
        ResponseActionBuilder::new()
    }

    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        url: impl Into<String>,
        method: Method,
    ) -> Self {
        Self {
            uuid: Uuid::prefixed("action"),
            title: title.into(),
            description: description.into(),
            url: url.into(),
            method: HTTPMethod::new(method),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResponseActionBuilder {
    uuid: Option<Uuid<Prefixed>>,

    title: Option<String>,
    description: Option<String>,

    url: Option<String>,
    method: HTTPMethod,
}

impl ResponseActionBuilder {
    pub fn new() -> Self {
        Self {
            uuid: None,

            title: None,
            description: None,

            url: None,
            method: HTTPMethod::new(Method::GET),
        }
    }

    pub fn with_uuid(mut self, uuid: Uuid<Prefixed>) -> Self {
        self.uuid = Some(uuid);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_method(mut self, method: Method) -> Self {
        self.method = HTTPMethod::new(method);
        self
    }

    pub fn build(self) -> RequestAction {
        RequestAction {
            uuid: self.uuid.unwrap_or(Uuid::prefixed("action")),
            title: self.title.unwrap(),
            description: self.description.unwrap(),
            url: self.url.unwrap(),
            method: self.method,
        }
    }
}
