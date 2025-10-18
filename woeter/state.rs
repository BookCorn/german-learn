use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use reqwest::Client;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::woeter::config::WoeterConfig;

#[derive(Clone)]
pub struct WoeterState {
    pub cfg: Arc<WoeterConfig>,
    pub notion_token: String,
    pub http: Client,
}

impl WoeterState {
    pub fn cors_layer() -> CorsLayer {
        CorsLayer::new()
            .allow_origin(HeaderValue::from_static("*"))
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([CONTENT_TYPE, AUTHORIZATION])
    }
}
