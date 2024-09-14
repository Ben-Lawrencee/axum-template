use serde::{Deserialize, Serialize};

use crate::uuid::Uuid;

use super::method::HTTPMethod;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestAction {
    pub uuid: Option<Uuid>,

    pub title: String,
    pub description: String,

    pub url: String,
    pub method: HTTPMethod,
}
