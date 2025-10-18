use std::sync::Arc;

use sea_orm::DatabaseConnection;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(db: DatabaseConnection, config: AppConfig) -> Self {
        Self { db, config }
    }

    pub fn into_shared(self) -> SharedState {
        Arc::new(self)
    }
}
