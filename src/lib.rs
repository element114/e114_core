#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

#[cfg(feature = "actix_web")]
pub mod actix_web;
#[cfg(feature = "hyper")]
pub mod hyper;
pub mod responses;
pub mod typed;

#[cfg(feature = "jsonschema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Skip as many entires as specified in @offset, default 0.
/// List at most as many entries as specified in @limit, default 100.
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListOptions {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<ListOrder>,
    pub sort: Option<String>,
}
impl Default for ListOptions {
    #[must_use]
    fn default() -> Self {
        Self { offset: None, limit: Some(100), order: None, sort: None }
    }
}

#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ListOrder {
    #[serde(alias = "ASC", alias = "asc")]
    Asc,
    #[serde(alias = "DESC", alias = "desc")]
    Desc,
}
