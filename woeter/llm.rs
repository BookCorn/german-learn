use anyhow::{Context, Result, anyhow};
use async_openai::types::{
    ChatChoice, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
};
use async_openai::{Client, config::OpenAIConfig};
use futures_util::StreamExt;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::woeter::config::LlmApi;
use crate::woeter::state::WoeterState;

pub async fn fetch_from_ai_model(
    state: &WoeterState,
    system_prompt: &str,
    user_input: &str,
    model_name: &str,
) -> Result<Vec<HashMap<String, Value>>> {
    // Choose API per model (fallback to global)
    let model_cfg = state
        .cfg
        .models
        .get(model_name)
        .ok_or_else(|| anyhow!("model '{}' not configured", model_name))?;
    let api = model_cfg
        .llm_api
        .clone()
        .unwrap_or(state.cfg.llm_api.clone());
    match api {
        LlmApi::ChatCompletions => {
            fetch_via_chat_completions(state, system_prompt, user_input, model_name).await
        }
        LlmApi::Responses => {
            fetch_via_responses_api(state, system_prompt, user_input, model_name).await
        }
    }
}

async fn fetch_via_chat_completions(
    state: &WoeterState,
    system_prompt: &str,
    user_input: &str,
    model_name: &str,
) -> Result<Vec<HashMap<String, Value>>> {
    let model_cfg = state
        .cfg
        .models
        .get(model_name)
        .ok_or_else(|| anyhow!("model '{}' not configured", model_name))?;

    let conf = OpenAIConfig::new()
        .with_api_key(model_cfg.api_key()?)
        .with_api_base(model_cfg.base_url.clone());
    let client = Client::with_config(conf);

    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: ChatCompletionRequestSystemMessageContent::Text(system_prompt.to_string()),
            name: None,
        }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(format!(
                "Please provide information for these German words: {}",
                user_input
            )),
            name: None,
        }),
    ];

    let req = CreateChatCompletionRequest {
        model: model_cfg.model_name.clone().into(),
        messages,
        temperature: Some(0.1),
        ..Default::default()
    };

    let start = std::time::Instant::now();
    let resp = client.chat().create(req).await?;
    tracing::info!(
        "HTTP (async-openai) chat/completions – ok in {:?}",
        start.elapsed()
    );

    let first: &ChatChoice = resp
        .choices
        .first()
        .ok_or_else(|| anyhow!("no choices in chat completion"))?;
    let content = first.message.content.clone().unwrap_or_default();

    let content = content.trim().to_string();
    let mut content = content;
    if content.starts_with("```json") {
        content = content.trim_start_matches("```json").to_string();
        if let Some(idx) = content.rfind("```") {
            content.truncate(idx);
        }
        content = content.trim().to_string();
    }

    if let Ok(pretty) = serde_json::from_str::<Value>(&content) {
        tracing::info!(
            "\nRaw JSON from {}:\n{}",
            model_name,
            serde_json::to_string_pretty(&pretty).unwrap_or_default()
        );
    }

    let entries: Vec<HashMap<String, Value>> = serde_json::from_str(&content)
        .map_err(|e| anyhow!("failed to parse JSON response: {}. Raw: {}", e, content))?;
    Ok(entries)
}

async fn fetch_via_responses_api(
    state: &WoeterState,
    system_prompt: &str,
    user_input: &str,
    model_name: &str,
) -> Result<Vec<HashMap<String, Value>>> {
    // Use raw HTTP for broad compatibility (many providers emulate OpenAI).
    let model_cfg = state
        .cfg
        .models
        .get(model_name)
        .ok_or_else(|| anyhow!("model '{}' not configured", model_name))?;

    let base = model_cfg.base_url.trim_end_matches('/');
    let url = if base.ends_with("/v1") {
        format!("{}/responses", base)
    } else {
        format!("{}/v1/responses", base)
    };

    // Minimal Responses API body with structured content
    // Depending on provider, either use top-level instructions or send system as first input
    let mut body = if state.cfg.responses.use_instructions {
        json!({
            "model": model_cfg.model_name,
            "instructions": system_prompt,
            "input": [
                {"role": "user", "content": [{"type": "input_text", "text": format!("Please provide information for these German words: {}", user_input)}]},
            ]
        })
    } else {
        json!({
            "model": model_cfg.model_name,
            "input": [
                {"role": "system", "content": [{"type": "input_text", "text": system_prompt}]},
                {"role": "user", "content": [{"type": "input_text", "text": format!("Please provide information for these German words: {}", user_input)}]},
            ]
        })
    };

    // reasoning effort
    if let Some(effort) = &state.cfg.responses.reasoning_effort {
        body.as_object_mut()
            .unwrap()
            .insert("reasoning".into(), json!({"effort": effort}));
    }

    let stream_enabled = state.cfg.responses.stream;
    if stream_enabled {
        body.as_object_mut()
            .unwrap()
            .insert("stream".into(), json!(true));
    }

    let start = std::time::Instant::now();
    let dbg = is_responses_debug_enabled(state);
    if dbg {
        tracing::info!(
            target: "woeter.responses",
            "[debug] responses request: url={}, model={}, stream={}, use_instructions={}, reasoning_effort={:?}\n{}",
            url,
            model_cfg.model_name,
            stream_enabled,
            state.cfg.responses.use_instructions,
            state.cfg.responses.reasoning_effort,
            serde_json::to_string_pretty(&body).unwrap_or_default()
        );
    }
    if stream_enabled {
        let resp = state
            .http
            .post(url)
            .header("Accept", "text/event-stream")
            .bearer_auth(model_cfg.api_key()?)
            .json(&body)
            .send()
            .await
            .context("responses request failed")?;
        let status = resp.status();
        if dbg {
            let ct = resp
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            tracing::info!(target: "woeter.responses", "[debug] responses stream status={}, content-type={}", status.as_u16(), ct);
        }
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("responses failed: {} - {}", status, text));
        }
        tracing::info!("HTTP POST responses (stream) – in {:?}", start.elapsed());

        let mut output_text = String::new();
        let mut buffer = Vec::<u8>::new();
        let mut stream = resp.bytes_stream();
        let mut line_count: usize = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.extend_from_slice(&chunk);
            // split by newlines
            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let line = buffer.drain(..=pos).collect::<Vec<u8>>();
                let line = String::from_utf8_lossy(&line).trim().to_string();
                if line.is_empty() {
                    continue;
                }
                if dbg && line_count < 300 {
                    tracing::info!(target: "woeter.responses", "[debug] raw-line: {}", line);
                    line_count += 1;
                }
                // SSE may include comments, event: name, and data: payload
                if let Some(data) = line.strip_prefix("data:") {
                    let data = data.trim();
                    // [DONE] sentinel or JSON
                    if data == "[DONE]" {
                        break;
                    }
                    if data.is_empty() {
                        continue;
                    }
                    if let Ok(val) = serde_json::from_str::<Value>(data) {
                        // Try to log reasoning summary or deltas
                        if let Some(reasoning) = extract_reasoning_text(&val) {
                            tracing::info!("[responses:reasoning] {}", reasoning);
                        }
                        if let Some(delta) = extract_output_text_delta(&val) {
                            output_text.push_str(&delta);
                        }
                    }
                } else {
                    // Some providers may send trailing JSON without the 'data:' prefix
                    if line.is_empty() {
                        continue;
                    }
                    if let Ok(val) = serde_json::from_str::<Value>(&line) {
                        if let Some(reasoning) = extract_reasoning_text(&val) {
                            tracing::info!("[responses:reasoning] {}", reasoning);
                        }
                        if let Some(delta) = extract_output_text_delta(&val) {
                            output_text.push_str(&delta);
                        }
                    }
                }
            }
        }

        // Flush remaining buffer as a final line if any
        if !buffer.is_empty() {
            let line = String::from_utf8_lossy(&buffer).trim().to_string();
            if dbg {
                tracing::info!(target: "woeter.responses", "[debug] flush-line: {}", line);
            }
            if let Some(data) = line.strip_prefix("data:") {
                let data = data.trim();
                if data != "[DONE]" {
                    if let Ok(val) = serde_json::from_str::<Value>(data) {
                        if let Some(reasoning) = extract_reasoning_text(&val) {
                            tracing::info!("[responses:reasoning] {}", reasoning);
                        }
                        if let Some(delta) = extract_output_text_delta(&val) {
                            output_text.push_str(&delta);
                        }
                    }
                }
            } else if !line.is_empty() {
                if let Ok(val) = serde_json::from_str::<Value>(&line) {
                    if let Some(reasoning) = extract_reasoning_text(&val) {
                        tracing::info!("[responses:reasoning] {}", reasoning);
                    }
                    if let Some(delta) = extract_output_text_delta(&val) {
                        output_text.push_str(&delta);
                    }
                }
            }
        }

        let content = output_text.trim().to_string();
        if dbg {
            tracing::info!(target: "woeter.responses", "[debug] aggregated-output-text (len={}): {}", content.len(), &content);
        }
        // Try to pretty log if content is JSON
        if let Ok(pretty) = serde_json::from_str::<Value>(&content) {
            tracing::info!(
                "\nRaw JSON from {} (responses-stream):\n{}",
                model_name,
                serde_json::to_string_pretty(&pretty).unwrap_or_default()
            );
        }
        let entries: Vec<HashMap<String, Value>> = serde_json::from_str(&content).map_err(|e| {
            anyhow!(
                "failed to parse JSON response (responses-stream): {}. Raw: {}",
                e,
                content
            )
        })?;
        return Ok(entries);
    } else {
        let resp = state
            .http
            .post(url)
            .bearer_auth(model_cfg.api_key()?)
            .json(&body)
            .send()
            .await
            .context("responses request failed")?;
        let status = resp.status();
        let text = resp.text().await.context("reading responses body")?;
        tracing::info!("HTTP POST responses – {} in {:?}", status, start.elapsed());
        if dbg {
            tracing::info!(target: "woeter.responses", "[debug] raw-body (len={}): {}", text.len(), &text);
        }
        if !status.is_success() {
            return Err(anyhow!("responses failed: {} - {}", status, text));
        }

        if text.trim().is_empty() {
            return Err(anyhow!("responses returned empty body"));
        }
        let v: Value = serde_json::from_str(&text)
            .or_else(|_| {
                // Some providers might send line-delimited JSON; try last non-empty line
                if let Some(last) = text.lines().rev().find(|l| !l.trim().is_empty()) {
                    serde_json::from_str::<Value>(last)
                } else {
                    Err(serde_json::Error::io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "empty lines",
                    )))
                }
            })
            .context("parsing responses JSON")?;
        if dbg {
            tracing::info!(target: "woeter.responses", "[debug] parsed-json-keys: top={:?}", v.as_object().map(|m| m.keys().cloned().collect::<Vec<_>>()));
        }
        if let Some(reasoning) = extract_reasoning_text(&v) {
            tracing::info!("[responses:reasoning] {}", reasoning);
        }

        // Try several extraction strategies depending on provider
        // 1) output_text at top-level
        // 2) iterate output[]; take first item that contains content[] with output_text/text/summary_text
        let content = v
            .get("output_text")
            .and_then(|x| x.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                v.get("output")
                    .and_then(|out| out.as_array())
                    .and_then(|arr| {
                        arr.iter().find_map(|elem| {
                            elem.get("content")
                                .and_then(|c| c.as_array())
                                .and_then(|carr| {
                                    carr.iter().find_map(|item| {
                                        if let Some(t) = item.get("type").and_then(|t| t.as_str()) {
                                            if t == "output_text" || t == "text" {
                                                return item
                                                    .get("text")
                                                    .and_then(|t| t.as_str())
                                                    .map(|s| s.to_string());
                                            }
                                            if t == "summary_text" {
                                                return item
                                                    .get("text")
                                                    .and_then(|t| t.as_str())
                                                    .map(|s| s.to_string());
                                            }
                                            if t == "reasoning" {
                                                if let Some(txt) =
                                                    item.get("text").and_then(|t| t.as_str())
                                                {
                                                    tracing::info!("[responses:reasoning] {}", txt);
                                                }
                                            }
                                        }
                                        None
                                    })
                                })
                        })
                    })
            });
        if dbg {
            tracing::info!(target: "woeter.responses", "[debug] extracted-content-present={}", content.as_ref().map(|s| !s.is_empty()).unwrap_or(false));
        }

        let mut content = content.ok_or_else(|| anyhow!("no text content in responses output"))?;
        content = content.trim().to_string();
        if content.starts_with("```json") {
            content = content.trim_start_matches("```json").to_string();
            if let Some(idx) = content.rfind("```") {
                content.truncate(idx);
            }
            content = content.trim().to_string();
        }

        if let Ok(pretty) = serde_json::from_str::<Value>(&content) {
            tracing::info!(
                "\nRaw JSON from {} (responses):\n{}",
                model_name,
                serde_json::to_string_pretty(&pretty).unwrap_or_default()
            );
        }
        let entries: Vec<HashMap<String, Value>> = serde_json::from_str(&content).map_err(|e| {
            anyhow!(
                "failed to parse JSON response (responses): {}. Raw: {}",
                e,
                content
            )
        })?;
        Ok(entries)
    }
}

fn is_responses_debug_enabled(state: &WoeterState) -> bool {
    if state.cfg.responses.debug {
        return true;
    }
    match std::env::var("WOETER_RESPONSES_DEBUG") {
        Ok(v) => matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "on"),
        Err(_) => false,
    }
}

fn extract_reasoning_text(v: &Value) -> Option<String> {
    // Look for top-level reasoning.summary
    if let Some(summary) = v
        .get("reasoning")
        .and_then(|r| r.get("summary"))
        .and_then(|s| s.as_str())
    {
        return Some(summary.to_string());
    }
    // Look in output[].content[] items with type == "reasoning"
    if let Some(out) = v.get("output").and_then(|o| o.as_array()) {
        for item in out {
            if let Some(content) = item.get("content").and_then(|c| c.as_array()) {
                for part in content {
                    if let Some(t) = part.get("type").and_then(|t| t.as_str()) {
                        if t == "reasoning" {
                            if let Some(txt) = part.get("text").and_then(|t| t.as_str()) {
                                return Some(txt.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn extract_output_text_delta(v: &Value) -> Option<String> {
    // For streaming: handle OpenAI-style Responses SSE events
    // Case A: {"type":"response.output_text.delta", "delta":"..."}
    if let Some(t) = v.get("type").and_then(|t| t.as_str()) {
        if t.ends_with("output_text.delta") {
            if let Some(s) = v.get("delta").and_then(|s| s.as_str()) {
                return Some(s.to_string());
            }
            // Some variants: {"delta":{"output_text":{"text":"..."}}}
            if let Some(s) = v
                .get("delta")
                .and_then(|d| d.get("output_text"))
                .and_then(|d| d.get("text"))
                .and_then(|s| s.as_str())
            {
                return Some(s.to_string());
            }
        }
    }
    // Case B: non-stream or full message piece present in a single event
    if let Some(output) = v.get("output").and_then(|o| o.as_array()) {
        // Sometimes event carries full output text parts
        for item in output {
            if let Some(content) = item.get("content").and_then(|c| c.as_array()) {
                for part in content {
                    if let Some(t) = part.get("type").and_then(|t| t.as_str()) {
                        if t == "output_text" || t == "text" {
                            if let Some(txt) = part.get("text").and_then(|t| t.as_str()) {
                                return Some(txt.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
