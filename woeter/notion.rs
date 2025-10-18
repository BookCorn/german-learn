use anyhow::{Context, Result, anyhow};
use serde_json::{Value, json};

use crate::woeter::state::WoeterState;

pub async fn entry_exists(state: &WoeterState, db_id: &str, word_title: &str) -> Result<bool> {
    let url = format!("https://api.notion.com/v1/databases/{}/query", db_id);
    let title_prop = &state.cfg.notion.properties.title;
    let mut filter = serde_json::Map::new();
    filter.insert("property".into(), Value::String(title_prop.name.clone()));
    let mode = if title_prop.r#type == "title" {
        "title"
    } else {
        "rich_text"
    };
    filter.insert(mode.into(), json!({"equals": word_title}));
    let body = json!({
        "filter": Value::Object(filter),
        "page_size": 1
    });

    let start = std::time::Instant::now();
    let resp = state
        .http
        .post(url)
        .bearer_auth(&state.notion_token)
        .header("Notion-Version", "2022-06-28")
        .json(&body)
        .send()
        .await
        .context("notion query request failed")?;
    let status = resp.status();
    let v: Value = resp.json().await.context("reading notion query body")?;
    tracing::info!(
        "HTTP POST notion query – {} in {:?}",
        status,
        start.elapsed()
    );
    if !status.is_success() {
        return Err(anyhow!("notion query failed: {} - {}", status, v));
    }
    let count = v
        .get("results")
        .and_then(|r| r.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    Ok(count > 0)
}

fn rt_text(s: &str) -> Value {
    json!({"rich_text": [{"text": {"content": s}}]})
}

pub async fn insert_entry(
    state: &WoeterState,
    db_id: &str,
    word_type: &str,
    data: &std::collections::HashMap<String, Value>,
) -> Result<()> {
    let url = "https://api.notion.com/v1/pages";

    let mut properties = serde_json::Map::new();

    // Title property
    let title_cfg = &state.cfg.notion.properties.title;
    let word = data.get("Wörter").and_then(|v| v.as_str()).unwrap_or("");
    properties.insert(
        title_cfg.name.clone(),
        match title_cfg.r#type.as_str() {
            "title" => json!({"title": [{"text": {"content": word}}]}),
            "rich_text" => rt_text(word),
            _ => json!({"title": [{"text": {"content": word}}]}),
        },
    );

    // Genus
    if let Some(genus_cfg) = &state.cfg.notion.properties.genus {
        if let Some(genus) = data.get("Genus").and_then(|v| v.as_str()) {
            let val = match genus_cfg.r#type.as_str() {
                "select" => json!({"select": {"name": genus}}),
                "multi_select" => {
                    let parts = genus
                        .split(|c| c == ',' || c == '/' || c == '、')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| json!({"name": s}))
                        .collect::<Vec<_>>();
                    json!({"multi_select": parts})
                }
                "rich_text" => rt_text(genus),
                _ => json!({"select": {"name": genus}}),
            };
            properties.insert(genus_cfg.name.clone(), val);
        }
    }

    // Plural
    if let Some(plural_cfg) = &state.cfg.notion.properties.plural {
        if let Some(plural) = data.get("Plural").and_then(|v| v.as_str()) {
            let val = match plural_cfg.r#type.as_str() {
                "rich_text" => rt_text(plural),
                "title" => json!({"title": [{"text": {"content": plural}}]}),
                _ => rt_text(plural),
            };
            properties.insert(plural_cfg.name.clone(), val);
        }
    }

    // Meanings
    if let Some(cfg) = &state.cfg.notion.properties.meaning_cn {
        if let Some(v) = data.get("释义").and_then(|v| v.as_str()) {
            let val = match cfg.r#type.as_str() {
                "rich_text" => rt_text(v),
                "title" => json!({"title": [{"text": {"content": v}}]}),
                _ => rt_text(v),
            };
            properties.insert(cfg.name.clone(), val);
        }
    }
    if let Some(cfg) = &state.cfg.notion.properties.meaning_en {
        if let Some(v) = data.get("English").and_then(|v| v.as_str()) {
            let val = match cfg.r#type.as_str() {
                "rich_text" => rt_text(v),
                "title" => json!({"title": [{"text": {"content": v}}]}),
                _ => rt_text(v),
            };
            properties.insert(cfg.name.clone(), val);
        }
    }

    // Beispiele
    if let Some(cfg) = &state.cfg.notion.properties.examples {
        if let Some(examples) = data.get("Beispiel") {
            let joined = match examples {
                Value::String(s) => s.to_string(),
                Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join("\n"),
                _ => String::new(),
            };
            if !joined.is_empty() {
                let val = match cfg.r#type.as_str() {
                    "rich_text" => rt_text(&joined),
                    "title" => json!({"title": [{"text": {"content": joined}}]}),
                    _ => rt_text(&joined),
                };
                properties.insert(cfg.name.clone(), val);
            }
        }
    }

    // Attributes (Eigenschaft)
    if let Some(cfg) = &state.cfg.notion.properties.attributes {
        if let Some(attrs) = data.get("Eigenschaft") {
            let names: Vec<String> = match attrs {
                Value::Array(a) => a
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect(),
                Value::String(s) => vec![s.to_string()],
                _ => vec![],
            };
            if !names.is_empty() {
                let val = match cfg.r#type.as_str() {
                    "multi_select" => {
                        json!({"multi_select": names.into_iter().map(|n| json!({"name": n})).collect::<Vec<_>>()})
                    }
                    "select" => json!({"select": {"name": names[0]}}),
                    _ => rt_text(&names.join(", ")),
                };
                properties.insert(cfg.name.clone(), val);
            }
        }
    }

    // Optional: store word type if enabled
    if let Some(cfg) = &state.cfg.notion.properties.word_type {
        if cfg.enabled {
            let val = match cfg.r#type.as_str() {
                "select" => json!({"select": {"name": word_type}}),
                "multi_select" => json!({"multi_select": [{"name": word_type}]}),
                _ => rt_text(word_type),
            };
            properties.insert(cfg.name.clone(), val);
        }
    }

    // Komparativ & Superlativ
    if let Some(cfg) = &state.cfg.notion.properties.comparison {
        if let Some(v) = data.get("Komparativ & Superlativ").and_then(|v| v.as_str()) {
            let val = match cfg.r#type.as_str() {
                "rich_text" => rt_text(v),
                "title" => json!({"title": [{"text": {"content": v}}]}),
                _ => rt_text(v),
            };
            properties.insert(cfg.name.clone(), val);
        }
    }

    let body = json!({ "parent": {"database_id": db_id}, "properties": properties });

    let start = std::time::Instant::now();
    let resp = state
        .http
        .post(url)
        .bearer_auth(&state.notion_token)
        .header("Notion-Version", "2022-06-28")
        .json(&body)
        .send()
        .await
        .context("notion create page failed")?;
    let status = resp.status();
    let txt = resp.text().await.unwrap_or_default();
    tracing::info!(
        "HTTP POST notion create page – {} in {:?}",
        status,
        start.elapsed()
    );
    if !status.is_success() {
        return Err(anyhow!("notion create failed: {} - {}", status, txt));
    }
    Ok(())
}
