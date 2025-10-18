use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};

use crate::{auth::validate_token, error::AppError, state::SharedState};
use axum::http::HeaderMap;

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
    headers: HeaderMap,
) -> Result<Json<Option<FlashcardResponse>>, AppError> {
    let service = FlashcardService::new(state.clone());
    let user = crate::auth::current_user_from_headers(&headers, &state)?;
    let user_id = user.user_id;
    let card = service.get_next_card(&user_id, params).await?;
    Ok(Json(card))
}

async fn get_stats(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<StatsResponse>, AppError> {
    let service = FlashcardService::new(state.clone());
    let user = crate::auth::current_user_from_headers(&headers, &state)?;
    let user_id = user.user_id;
    let stats = service.get_stats(&user_id).await?;
    Ok(Json(stats))
}

async fn post_review(
    State(state): State<SharedState>,
    Path(entry_id): Path<i32>,
    headers: HeaderMap,
    Json(payload): Json<ReviewRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let service = FlashcardService::new(state.clone());
    let user = crate::auth::current_user_from_headers(&headers, &state)?;
    let user_id = user.user_id;
    service.record_review(&user_id, entry_id, payload).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}
