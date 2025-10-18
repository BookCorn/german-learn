use axum::{extract::{State, Path, Query}, routing::{post, get, patch, delete}, Json, Router};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, prelude::DateTimeWithTimeZone, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::SharedState, entity::vocabulary_entries, auth::current_user_from_headers};
use axum::http::HeaderMap;

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub word: String,
    pub part_of_speech: String,
    #[serde(default)]
    pub english: Option<String>,
    #[serde(default)]
    pub meaning: Option<String>,
    #[serde(default)]
    pub examples: Option<String>,
    #[serde(default)]
    pub themes: Option<String>,
    #[serde(default)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateEntryResponse { pub entry_id: i32 }

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/api/v1/entries", post(create_entry))
        .route("/api/v1/entries/mine", get(list_my_entries))
        .route("/api/v1/entries/ai-fill", post(ai_fill_entries))
        .route("/api/v1/entries/{entry_id}", patch(update_entry).delete(delete_entry))
        .with_state(state)
}

async fn create_entry(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(req): Json<CreateEntryRequest>,
) -> Result<Json<CreateEntryResponse>, AppError> {
    let user = current_user_from_headers(&headers, &state)?;
    let pos = normalize_part_of_speech(&req.part_of_speech)?;
    let now: DateTimeWithTimeZone = Utc::now().into();

    let model = vocabulary_entries::ActiveModel {
        entry_id: sea_orm::ActiveValue::NotSet,
        word: Set(req.word.trim().to_string()),
        part_of_speech: Set(pos),
        user_owner: Set(Some(user.user_id.clone())),
        english: Set(req.english.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())),
        meaning: Set(req.meaning.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())),
        examples: Set(req.examples.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())),
        themes: Set(req.themes.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())),
        source_table: Set("user".to_string()),
        source_created_time: Set(Some(now.clone())),
        extra: Set(req.extra.map(Into::into)),
    };

    let inserted = model.insert(&state.db).await?;
    Ok(Json(CreateEntryResponse { entry_id: inserted.entry_id }))
}

#[derive(Debug, Serialize)]
struct MyEntryItem {
    entry_id: i32,
    word: String,
    part_of_speech: String,
    english: Option<String>,
    meaning: Option<String>,
    examples: Option<String>,
    themes: Option<String>,
    extra: Option<serde_json::Value>,
}

async fn list_my_entries(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<Vec<MyEntryItem>>, AppError> {
    let user = current_user_from_headers(&headers, &state)?;
    let rows = vocabulary_entries::Entity::find()
        .filter(vocabulary_entries::Column::UserOwner.eq(user.user_id))
        .all(&state.db)
        .await?;

    let items = rows.into_iter().map(|e| MyEntryItem {
        entry_id: e.entry_id,
        word: e.word,
        part_of_speech: e.part_of_speech,
        english: e.english,
        meaning: e.meaning,
        examples: e.examples,
        themes: e.themes,
        extra: e.extra.map(|j| j.into()),
    }).collect();

    Ok(Json(items))
}

#[derive(Debug, Deserialize)]
struct UpdateEntryRequest {
    #[serde(default)] word: Option<String>,
    #[serde(default)] part_of_speech: Option<String>,
    #[serde(default)] english: Option<String>,
    #[serde(default)] meaning: Option<String>,
    #[serde(default)] examples: Option<String>,
    #[serde(default)] themes: Option<String>,
    #[serde(default)] extra: Option<serde_json::Value>,
}

async fn update_entry(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(entry_id): Path<i32>,
    Json(req): Json<UpdateEntryRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = current_user_from_headers(&headers, &state)?;
    let mut model = match vocabulary_entries::Entity::find()
        .filter(vocabulary_entries::Column::EntryId.eq(entry_id))
        .filter(vocabulary_entries::Column::UserOwner.eq(user.user_id.clone()))
        .one(&state.db)
        .await? {
            Some(m) => m,
            None => return Err(AppError::NotFound),
        };

    let mut active: vocabulary_entries::ActiveModel = model.clone().into();
    if let Some(w) = req.word { active.word = Set(w); }
    if let Some(pos) = req.part_of_speech { active.part_of_speech = Set(normalize_part_of_speech(&pos)?); }
    if let Some(v) = req.english { active.english = Set(Some(v)); }
    if let Some(v) = req.meaning { active.meaning = Set(Some(v)); }
    if let Some(v) = req.examples { active.examples = Set(Some(v)); }
    if let Some(v) = req.themes { active.themes = Set(Some(v)); }
    if let Some(v) = req.extra { active.extra = Set(Some(v.into())); }
    active.update(&state.db).await?;
    Ok(Json(serde_json::json!({"status":"ok"})))
}

async fn delete_entry(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(entry_id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = current_user_from_headers(&headers, &state)?;
    let res = vocabulary_entries::Entity::delete_many()
        .filter(vocabulary_entries::Column::EntryId.eq(entry_id))
        .filter(vocabulary_entries::Column::UserOwner.eq(user.user_id))
        .exec(&state.db)
        .await?;
    if res.rows_affected == 0 { return Err(AppError::NotFound); }
    Ok(Json(serde_json::json!({"status":"ok"})))
}

fn normalize_part_of_speech(input: &str) -> Result<String, AppError> {
    let normalized = input.trim().to_lowercase();
    let mapped = match normalized.as_str() {
        "noun" | "n" => "noun",
        "verb" | "v" => "verb",
        "adjective" | "adj" | "adverb" | "adv" | "adjective_adverb" => "adjective_adverb",
        other => {
            return Err(AppError::Validation(format!(
                "unsupported part_of_speech '{}'",
                other
            )));
        }
    };
    Ok(mapped.to_string())
}

// -------------------- AI Fill (async-openai via woeter) ----------------------
use async_openai::{Client, config::OpenAIConfig};
use async_openai::types::{CreateChatCompletionRequest, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize)]
struct AiFillRequest {
    part_of_speech: String,          // noun | verb | adjective_adverb
    words: String,                   // comma/换行/分号分隔
    #[serde(default)] model: Option<String>,
}

#[derive(Debug, Serialize)]
struct AiFillResponseItem { word: String, status: String, message: Option<String>, entry_id: Option<i32> }
#[derive(Debug, Serialize)]
struct AiFillResponse { ok: bool, model: String, items: Vec<AiFillResponseItem> }

async fn ai_fill_entries(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(req): Json<AiFillRequest>,
) -> Result<Json<AiFillResponse>, AppError> {
    let user = current_user_from_headers(&headers, &state)?;
    let pos = normalize_part_of_speech(&req.part_of_speech)?;

    // Build minimal woeter state from env for LLM calls
    let (prompt, _) = match pos.as_str() {
        "noun" => (SYSTEM_PROMPT_NOUN, "Noun"),
        "verb" => (SYSTEM_PROMPT_VERB, "Verb"),
        _ => (SYSTEM_PROMPT_ADJ_ADV, "Adjective/Adverb"),
    };
    let cfg = load_ai_provider_config();
    let model_to_use = req
        .model
        .clone()
        .or_else(|| cfg.as_ref().and_then(|c| c.model.clone()))
        .or_else(|| std::env::var("OPENAI_MODEL").ok())
        .unwrap_or_else(|| "gpt-4o-mini".to_string());

    let words: Vec<String> = req
        .words
        .split(|c: char| c == ',' || c == '\n' || c == ';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    if words.is_empty() {
        return Err(AppError::Validation("words is empty".into()));
    }
    let query = words.join(", ");
    let entries = fetch_entries_via_chat(prompt, &query, &model_to_use).await?;

    let mut items = Vec::new();
    for entry in entries {
        let word = entry.get("Wörter").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
        if word.is_empty() {
            items.push(AiFillResponseItem{ word: "".into(), status:"error".into(), message: Some("missing 'Wörter' field".into()), entry_id: None});
            continue;
        }
        // Build model
        let now: DateTimeWithTimeZone = Utc::now().into();
        let mut english = entry.get("English").and_then(|v| v.as_str()).map(|s| s.to_string());
        let mut meaning = entry.get("释义").and_then(|v| v.as_str()).map(|s| s.to_string());
        let examples = match entry.get("Beispiel") {
            Some(JsonValue::Array(a)) => Some(
                a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("\n")
            ),
            Some(JsonValue::String(s)) => Some(s.to_string()),
            _ => None,
        };

        let extra = match pos.as_str() {
            "noun" => {
                let gender = entry.get("Genus").and_then(|v| v.as_str()).map(|s| s.to_string());
                let plural = entry.get("Plural").and_then(|v| v.as_str()).map(|s| s.to_string());
                serde_json::json!({"gender": gender, "plural": plural})
            }
            "verb" => {
                let props = match entry.get("Eigenschaft") {
                    Some(JsonValue::Array(a)) => a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", "),
                    Some(JsonValue::String(s)) => s.clone(),
                    _ => String::new(),
                };
                serde_json::json!({"properties": if props.is_empty(){ serde_json::Value::Null } else { serde_json::Value::String(props) }})
            }
            _ => {
                let cmp = entry.get("Komparativ & Superlativ").and_then(|v| v.as_str()).unwrap_or("");
                let forms: Vec<String> = cmp.split(|c| c==',' || c==';').map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                let attribute = entry.get("属性").and_then(|v| v.as_str()).map(|s| s.to_string());
                serde_json::json!({"attribute": attribute, "comparison_forms": forms})
            }
        };

        // Insert to DB
        let model = vocabulary_entries::ActiveModel {
            entry_id: sea_orm::ActiveValue::NotSet,
            word: Set(word.clone()),
            part_of_speech: Set(pos.clone()),
            user_owner: Set(Some(user.user_id.clone())),
            english: Set(english.take()),
            meaning: Set(meaning.take()),
            examples: Set(examples),
            themes: Set(None),
            source_table: Set("user_ai".to_string()),
            source_created_time: Set(Some(now.clone())),
            extra: Set(Some(extra.into())),
        };
        match model.insert(&state.db).await {
            Ok(inserted) => items.push(AiFillResponseItem{ word, status:"inserted".into(), message: None, entry_id: Some(inserted.entry_id)}),
            Err(e) => items.push(AiFillResponseItem{ word, status:"error".into(), message: Some(e.to_string()), entry_id: None}),
        }
    }

    Ok(Json(AiFillResponse{ ok: true, model: model_to_use, items }))
}

const SYSTEM_PROMPT_NOUN: &str = include_str!("../../woeter/prompts_noun.txt");
const SYSTEM_PROMPT_VERB: &str = include_str!("../../woeter/prompts_verb.txt");
const SYSTEM_PROMPT_ADJ_ADV: &str = include_str!("../../woeter/prompts_adj_adv.txt");

async fn fetch_entries_via_chat(system_prompt:&str, user_input:&str, model:&str) -> Result<Vec<std::collections::HashMap<String, JsonValue>>, AppError> {
    let cfg = load_ai_provider_config();
    let api_key = cfg.as_ref().and_then(|c| c.api_key.clone()).or_else(|| std::env::var("OPENAI_API_KEY").ok()).or_else(|| std::env::var("WOETER_OPENAI_KEY").ok()).ok_or_else(|| AppError::Validation("Missing OPENAI_API_KEY or config/ai.toml api_key".into()))?;
    let base_url = cfg.as_ref().and_then(|c| c.base_url.clone()).or_else(|| std::env::var("OPENAI_BASE_URL").ok()).unwrap_or_else(|| "https://api.openai.com/v1".to_string());
    let conf = OpenAIConfig::new().with_api_key(api_key).with_api_base(base_url);
    let client = Client::with_config(conf);
    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage{ content: ChatCompletionRequestSystemMessageContent::Text(system_prompt.to_string()), name: None }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage{ content: ChatCompletionRequestUserMessageContent::Text(format!("Please provide information for these German words: {}", user_input)), name: None }),
    ];
    let req = CreateChatCompletionRequest {
        model: model.to_string().into(),
        messages,
        temperature: Some(0.1),
        max_tokens: Some(800),
        ..Default::default()
    };
    let resp = match client.chat().create(req).await {
        Ok(r) => r,
        Err(e) => {
            // 将外部 API 错误下沉为 400，方便前端展示具体原因
            return Err(AppError::Validation(format!("chat completions error: {}", e)));
        }
    };
    let content = resp.choices.first().and_then(|c| c.message.content.clone()).unwrap_or_default();
    let mut content = content.trim().to_string();
    if content.starts_with("```json") { content = content.trim_start_matches("```json").to_string(); if let Some(i)=content.rfind("```") { content.truncate(i); } content = content.trim().to_string(); }
    // Try strict parse first; if fails, try to salvage the first top-level JSON array substring
    let parsed: Vec<std::collections::HashMap<String, JsonValue>> = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            if let Some(slice) = extract_json_array(&content) {
                serde_json::from_str(slice).map_err(|e2| AppError::Validation(format!("LLM 返回不是合法 JSON（数组）。err1={e}; err2={e2}")))?
            } else {
                return Err(AppError::Validation(format!("LLM 返回不是合法 JSON（数组）。err={e}")));
            }
        }
    };
    Ok(parsed)
}

#[derive(serde::Deserialize)]
struct AiProviderConfig { base_url: Option<String>, api_key: Option<String>, model: Option<String> }
fn load_ai_provider_config() -> Option<AiProviderConfig> {
    let candidates = ["config/ai.toml", "ai.toml", "woeter/ai.toml"]; 
    for p in candidates {
        if let Ok(s) = std::fs::read_to_string(p) { if let Ok(v) = toml::from_str::<AiProviderConfig>(&s) { return Some(v); } }
    }
    None
}

fn extract_json_array(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut start: Option<usize> = None;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'[' {
            if depth == 0 { start = Some(i); }
            depth += 1;
        } else if b == b']' {
            if depth > 0 { depth -= 1; }
            if depth == 0 {
                if let Some(st) = start { return Some(&s[st..=i]); }
            }
        }
    }
    None
}
