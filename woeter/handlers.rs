use anyhow::Result;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::woeter::llm::fetch_from_ai_model;
use crate::woeter::notion::{entry_exists, insert_entry};
use crate::woeter::state::WoeterState;

#[derive(Deserialize)]
pub struct AddWordRequest {
    pub word_type: i32, // 1=noun, 2=verb, 3=adj_adv, 4=prep_reflex
    pub words: String,  // comma/newline separated
    #[serde(default)]
    pub model: Option<String>,
}

#[derive(Serialize)]
pub struct AddWordResponse {
    pub success: bool,
    pub message: String,
    pub results: Vec<ResultEntry>,
    #[serde(rename = "model_used")]
    pub model: String,
}

#[derive(Serialize)]
pub struct ResultEntry {
    pub word: String,
    pub status: String, // inserted | duplicate | error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub async fn add_words(
    State(state): State<WoeterState>,
    Json(req): Json<AddWordRequest>,
) -> impl IntoResponse {
    let (db_id, system_prompt, word_type_name) = match get_word_type_info(&state, req.word_type) {
        Ok(info) => info,
        Err(e) => {
            let body = Json(json!({"success": false, "message": e.to_string()}));
            return (StatusCode::BAD_REQUEST, body).into_response();
        }
    };

    let model_to_use = req
        .model
        .and_then(|m| state.cfg.models.contains_key(&m).then_some(m))
        .unwrap_or_else(|| state.cfg.default_model.clone());
    if !state.cfg.models.contains_key(&model_to_use) {
        let body = Json(json!({
            "success": false,
            "message": format!(
                "Model '{}' not available. Available models: {:?}",
                model_to_use,
                available_models(&state)
            )
        }));
        return (StatusCode::BAD_REQUEST, body).into_response();
    }

    // 1) Parse input words and drop duplicates in the list
    let mut results: Vec<ResultEntry> = Vec::new();
    let mut seen = std::collections::HashSet::<String>::new();
    let mut uniques: Vec<String> = Vec::new();
    for raw in split_words(&req.words) {
        if seen.insert(raw.clone()) {
            uniques.push(raw);
        } else {
            results.push(ResultEntry {
                word: raw,
                status: "duplicate".into(),
                message: Some("duplicated in input".into()),
            });
        }
    }

    // 2) Pre-check against Notion: remove existing words before calling LLM
    let mut to_query: Vec<String> = Vec::new();
    for w in uniques {
        match entry_exists(&state, &db_id, &w).await {
            Ok(true) => {
                info!("[SKIP] '{}' exists in Notion; skipping LLM", w);
                results.push(ResultEntry {
                    word: w,
                    status: "duplicate".into(),
                    message: Some("exists in Notion".into()),
                });
            }
            Ok(false) => to_query.push(w),
            Err(e) => {
                error!("Pre-check error for '{}': {}", w, e);
                results.push(ResultEntry {
                    word: w,
                    status: "error".into(),
                    message: Some(format!("precheck error: {}", e)),
                });
            }
        }
    }

    // If nothing to query after pre-filter, return early
    if to_query.is_empty() {
        let resp = AddWordResponse {
            success: true,
            message: format!(
                "Processed {} words using {} model",
                results.len(),
                model_to_use
            ),
            results,
            model: model_to_use,
        };
        return (StatusCode::OK, Json(resp)).into_response();
    }

    let input_for_llm = to_query.join(", ");
    let mut llm_results = process_words(
        &state,
        &input_for_llm,
        &db_id,
        &system_prompt,
        word_type_name,
        &model_to_use,
    )
    .await;
    results.append(&mut llm_results);

    let resp = AddWordResponse {
        success: true,
        message: format!(
            "Processed {} words using {} model",
            results.len(),
            model_to_use
        ),
        results,
        model: model_to_use,
    };
    (StatusCode::OK, Json(resp)).into_response()
}

pub async fn models(State(state): State<WoeterState>) -> impl IntoResponse {
    let body = Json(json!({
        "available_models": available_models(&state),
        "default_model": state.cfg.default_model,
        "word_types": {"noun": 1, "verb": 2, "adj_adv": 3, "prep_reflex": 4}
    }));
    (StatusCode::OK, body)
}

pub async fn health(State(state): State<WoeterState>) -> impl IntoResponse {
    let body = Json(json!({
        "status": "healthy",
        "time": chrono::Utc::now().to_rfc3339(),
        "models": available_models(&state),
    }));
    (StatusCode::OK, body)
}

fn available_models(state: &WoeterState) -> Vec<String> {
    state.cfg.models.keys().cloned().collect()
}

fn get_word_type_info(
    state: &WoeterState,
    word_type: i32,
) -> Result<(String, String, &'static str)> {
    match word_type {
        1 => Ok((
            state.cfg.noun_db_id.clone(),
            state.cfg.prompts.noun.clone(),
            "Noun",
        )),
        2 => Ok((
            state.cfg.verb_db_id.clone(),
            state.cfg.prompts.verb.clone(),
            "Verb",
        )),
        3 => Ok((
            state.cfg.adj_adv_db_id.clone(),
            state.cfg.prompts.adj_adv.clone(),
            "Adjective/Adverb",
        )),
        4 => Ok((
            state.cfg.prep_reflex_db_id.clone(),
            state.cfg.prompts.prep_reflex.clone(),
            "Prepositional/Reflexive Verb",
        )),
        _ => anyhow::bail!(
            "invalid word type. Must be 1 (noun), 2 (verb), 3 (adj/adv), or 4 (prep/reflex)"
        ),
    }
}

async fn process_words(
    state: &WoeterState,
    input: &str,
    db_id: &str,
    system_prompt: &str,
    word_type_name: &str,
    model_name: &str,
) -> Vec<ResultEntry> {
    let mut results = Vec::new();
    if input.trim().is_empty() {
        return results;
    }

    info!("Calling {} model for: {}", model_name, input);
    let entries = match fetch_from_ai_model(state, system_prompt, input, model_name).await {
        Ok(v) => v,
        Err(e) => {
            error!("Error fetching from {}: {}", model_name, e);
            results.push(ResultEntry {
                word: input.to_string(),
                status: "error".into(),
                message: Some(format!("{} API error: {}", model_name, e)),
            });
            return results;
        }
    };

    info!("{} returned {} entries", model_name, entries.len());
    for entry in entries {
        let word = entry
            .get("Wörter")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if word.is_empty() {
            results.push(ResultEntry {
                word: "unknown".into(),
                status: "error".into(),
                message: Some("Missing 'Wörter' field in response".into()),
            });
            continue;
        }

        match entry_exists(state, db_id, &word).await {
            Ok(true) => {
                info!("[DUP] '{}' already exists.", word);
                results.push(ResultEntry {
                    word,
                    status: "duplicate".into(),
                    message: None,
                });
                continue;
            }
            Err(e) => {
                error!("Error checking for duplicate '{}': {}", word, e);
                results.push(ResultEntry {
                    word,
                    status: "error".into(),
                    message: Some(format!("Database check error: {}", e)),
                });
                continue;
            }
            _ => {}
        }

        match insert_entry(state, db_id, word_type_name, &entry).await {
            Ok(()) => {
                info!("[OK ] '{}' inserted successfully.", word);
                results.push(ResultEntry {
                    word,
                    status: "inserted".into(),
                    message: None,
                });
            }
            Err(e) => {
                error!("[ERR] Failed to insert '{}': {}", word, e);
                results.push(ResultEntry {
                    word,
                    status: "error".into(),
                    message: Some(format!("Insert error: {}", e)),
                });
            }
        }
    }

    results
}

fn split_words(input: &str) -> Vec<String> {
    input
        .split(|c: char| c == ',' || c == '\n' || c == ';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
