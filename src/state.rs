use std::sync::Arc;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn into_shared(self) -> SharedState {
        Arc::new(self)
    }
}
