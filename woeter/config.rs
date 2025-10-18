use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::{Context, anyhow};
use reqwest::Client;
use serde::Deserialize;

use crate::woeter::prompts::{
    default_prompt_adj_adv, default_prompt_noun, default_prompt_prep_reflex, default_prompt_verb,
};
use crate::woeter::state::WoeterState;

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
    #[serde(rename = "api_key_env")]
    pub api_key_env: Option<String>,
    #[serde(rename = "base_url")]
    pub base_url: String,
    #[serde(rename = "model_name")]
    pub model_name: String,
    #[serde(default)]
    pub llm_api: Option<LlmApi>,
}

impl ModelConfig {
    pub fn api_key(&self) -> anyhow::Result<String> {
        if let Some(env_name) = &self.api_key_env {
            if let Ok(v) = std::env::var(env_name) {
                if !v.is_empty() {
                    return Ok(v);
                }
            }
        }
        if let Some(v) = &self.api_key {
            if !v.is_empty() {
                return Ok(v.clone());
            }
        }
        anyhow::bail!(
            "API key not provided. Set 'api_key' or 'api_key_env' with a valid env var name."
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromptsConfig {
    #[serde(default = "default_prompt_noun")]
    pub noun: String,
    #[serde(default = "default_prompt_verb")]
    pub verb: String,
    #[serde(default = "default_prompt_adj_adv")]
    pub adj_adv: String,
    #[serde(default = "default_prompt_prep_reflex")]
    pub prep_reflex: String,
}

impl Default for PromptsConfig {
    fn default() -> Self {
        Self {
            noun: default_prompt_noun(),
            verb: default_prompt_verb(),
            adj_adv: default_prompt_adj_adv(),
            prep_reflex: default_prompt_prep_reflex(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropertyConfig {
    pub name: String,
    #[serde(default)]
    pub r#type: String, // title | rich_text | select | multi_select
    #[serde(default = "default_enabled_true")]
    pub enabled: bool,
}
fn default_enabled_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotionPropertiesConfig {
    #[serde(default = "default_prop_title")]
    pub title: PropertyConfig,
    #[serde(default = "default_prop_genus")]
    pub genus: Option<PropertyConfig>,
    #[serde(default = "default_prop_plural")]
    pub plural: Option<PropertyConfig>,
    #[serde(default = "default_prop_meaning_cn")]
    pub meaning_cn: Option<PropertyConfig>,
    #[serde(default = "default_prop_meaning_en")]
    pub meaning_en: Option<PropertyConfig>,
    #[serde(default = "default_prop_examples")]
    pub examples: Option<PropertyConfig>,
    #[serde(default = "default_prop_attributes")]
    pub attributes: Option<PropertyConfig>,
    #[serde(default = "default_prop_word_type")]
    pub word_type: Option<PropertyConfig>,
    #[serde(default = "default_prop_comparison")]
    pub comparison: Option<PropertyConfig>,
}
impl Default for NotionPropertiesConfig {
    fn default() -> Self {
        Self {
            title: default_prop_title(),
            genus: default_prop_genus(),
            plural: default_prop_plural(),
            meaning_cn: default_prop_meaning_cn(),
            meaning_en: default_prop_meaning_en(),
            examples: default_prop_examples(),
            attributes: default_prop_attributes(),
            word_type: default_prop_word_type(),
            comparison: default_prop_comparison(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NotionConfig {
    #[serde(default)]
    pub properties: NotionPropertiesConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WoeterConfig {
    #[serde(rename = "notion_noun_database_id")]
    pub noun_db_id: String,
    #[serde(rename = "notion_verb_database_id")]
    pub verb_db_id: String,
    #[serde(rename = "notion_adj_adv_database_id")]
    pub adj_adv_db_id: String,
    #[serde(rename = "notion_prep_reflex_database_id")]
    pub prep_reflex_db_id: String,
    pub models: HashMap<String, ModelConfig>,
    #[serde(default)]
    pub default_model: String,
    #[serde(default)]
    pub llm_api: LlmApi,
    #[serde(default)]
    pub responses: ResponsesConfig,
    #[serde(default)]
    pub prompts: PromptsConfig,
    #[serde(default)]
    pub notion: NotionConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmApi {
    ChatCompletions,
    Responses,
}

impl Default for LlmApi {
    fn default() -> Self {
        LlmApi::ChatCompletions
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ResponsesConfig {
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub reasoning_effort: Option<String>, // e.g., "low" | "medium" | "high"
    #[serde(default = "default_use_instructions_true")]
    pub use_instructions: bool,
    #[serde(default)]
    pub debug: bool,
}

fn default_use_instructions_true() -> bool {
    true
}

pub fn load_woeter_state() -> anyhow::Result<WoeterState> {
    let notion_token = std::env::var("NOTION_API_KEY")
        .context("please set NOTION_API_KEY environment variable")?;

    let cfg: WoeterConfig = {
        let path_env = std::env::var("WOETER_CONFIG").ok();
        let candidate_paths = if let Some(p) = path_env {
            vec![p]
        } else {
            vec![
                "config.toml".to_string(),
                "ledger-woeter/config.toml".to_string(),
                "../config.toml".to_string(),
            ]
        };
        let mut last_err = None;
        let mut loaded = None;
        for p in candidate_paths {
            match std::fs::read_to_string(&p) {
                Ok(data) => match toml::from_str::<WoeterConfig>(&data) {
                    Ok(cfg) => {
                        loaded = Some(cfg);
                        break;
                    }
                    Err(e) => {
                        last_err = Some(anyhow!(
                            "could not parse woeter TOML config at {}: {}",
                            p,
                            e
                        ))
                    }
                },
                Err(e) => last_err = Some(anyhow!("could not read {}: {}", p, e)),
            }
        }
        loaded.ok_or_else(|| last_err.unwrap_or_else(|| anyhow!("woeter TOML config not found")))?
    };

    if cfg.models.is_empty() {
        return Err(anyhow!("no models configured in woeter TOML config"));
    }

    let default_model = if cfg.default_model.is_empty() {
        cfg.models.keys().next().cloned().unwrap_or_default()
    } else {
        cfg.default_model.clone()
    };
    if !cfg.models.contains_key(&default_model) {
        return Err(anyhow!(
            "default model '{}' not found in models configuration",
            default_model
        ));
    }

    let cfg = WoeterConfig {
        default_model,
        ..cfg
    };
    Ok(WoeterState {
        cfg: Arc::new(cfg),
        notion_token,
        http: Client::builder().timeout(Duration::from_secs(60)).build()?,
    })
}

fn default_prop_title() -> PropertyConfig {
    PropertyConfig {
        name: "Wörter".to_string(),
        r#type: "title".to_string(),
        enabled: true,
    }
}
fn default_prop_genus() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "Genus".to_string(),
        r#type: "multi_select".to_string(),
        enabled: true,
    })
}
fn default_prop_plural() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "Plural".to_string(),
        r#type: "rich_text".to_string(),
        enabled: true,
    })
}
fn default_prop_meaning_cn() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "释义".to_string(),
        r#type: "rich_text".to_string(),
        enabled: true,
    })
}
fn default_prop_meaning_en() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "English".to_string(),
        r#type: "rich_text".to_string(),
        enabled: true,
    })
}
fn default_prop_examples() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "Beispiel".to_string(),
        r#type: "rich_text".to_string(),
        enabled: true,
    })
}
fn default_prop_attributes() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "Eigenschaft".to_string(),
        r#type: "multi_select".to_string(),
        enabled: true,
    })
}
fn default_prop_word_type() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "类型".to_string(),
        r#type: "select".to_string(),
        enabled: false,
    })
}
fn default_prop_comparison() -> Option<PropertyConfig> {
    Some(PropertyConfig {
        name: "Komparativ & Superlativ".to_string(),
        r#type: "rich_text".to_string(),
        enabled: true,
    })
}
