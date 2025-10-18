pub mod config;
pub mod handlers;
pub mod llm;
pub mod notion;
pub mod prompts;
pub mod state;

pub use config::{WoeterConfig, load_woeter_state};
pub use state::WoeterState;

use axum::extract::FromRef;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    WoeterState: FromRef<S> + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/add-words", post(handlers::add_words))
        .route("/models", get(handlers::models))
        .route("/woeter/health", get(handlers::health))
        .layer(TraceLayer::new_for_http())
        .layer(WoeterState::cors_layer())
}
