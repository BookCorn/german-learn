use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};

use crate::{error::AppError, state::SharedState};

use super::{
    dto::{FlashcardResponse, NextCardQuery, ReviewRequest, StatsResponse},
    service::FlashcardService,
};

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/api/v1/flashcards/next", get(get_next_flashcard))
        .route("/api/v1/flashcards/stats", get(get_stats))
        .route("/api/v1/flashcards/{entry_id}/review", post(post_review))
        .with_state(state)
}

async fn get_next_flashcard(
    State(state): State<SharedState>,
    Query(params): Query<NextCardQuery>,
) -> Result<Json<Option<FlashcardResponse>>, AppError> {
    let service = FlashcardService::new(state.clone());
    let card = service.get_next_card(params).await?;
    Ok(Json(card))
}

async fn get_stats(State(state): State<SharedState>) -> Result<Json<StatsResponse>, AppError> {
    let service = FlashcardService::new(state.clone());
    let stats = service.get_stats().await?;
    Ok(Json(stats))
}

async fn post_review(
    State(state): State<SharedState>,
    Path(entry_id): Path<i32>,
    Json(payload): Json<ReviewRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let service = FlashcardService::new(state.clone());
    service.record_review(entry_id, payload).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}
