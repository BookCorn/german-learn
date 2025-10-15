mod config;
mod error;
mod flashcard;
mod state;

mod entity;

use anyhow::Context;
use axum::routing::get;
use config::AppConfig;
use dotenvy::dotenv;
use sea_orm::Database;
use state::AppState;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "german_learn=debug,axum::rejection=trace,tower_http=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env().context("loading configuration")?;
    let db = Database::connect(&config.database_url)
        .await
        .context("connecting to database")?;
    let shared_state = AppState::new(db).into_shared();

    let app = flashcard::router(shared_state.clone())
        .route("/health", get(healthcheck))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .with_context(|| format!("binding to {}", config.server_addr))?;
    tracing::info!("listening on {}", config.server_addr);
    axum::serve(listener, app.into_make_service())
        .await
        .context("starting http server")
}

async fn healthcheck() -> &'static str {
    "ok"
}
