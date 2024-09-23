use std::str::FromStr;

use axum::http::Method;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HTTPMethod(Method);

impl HTTPMethod {
    pub fn new(method: Method) -> Self {
        Self(method)
    }
}

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HTTPMethod> for Method {
    fn from(method: HTTPMethod) -> Self {
        method.0
    }
}

impl Serialize for HTTPMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}

impl<'de> Deserialize<'de> for HTTPMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(Method::from_str(&s).unwrap()))
    }
}
