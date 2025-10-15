use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::entity::{flashcard_progress, vocabulary_entries};

#[derive(Debug, Serialize)]
pub struct FlashcardResponse {
    pub entry_id: i32,
    pub word: String,
    pub part_of_speech: String,
    pub meaning: Option<String>,
    pub english: Option<String>,
    pub examples: Option<String>,
    pub themes: Option<String>,
    pub status: Option<String>,
    pub times_seen: i32,
    pub times_mastered: i32,
    pub last_seen_at: Option<String>,
    pub metadata: Option<FlashcardMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewRequest {
    pub result: String,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct NextCardQuery {
    #[serde(default)]
    pub part_of_speech: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartOfSpeechStats {
    pub part_of_speech: String,
    pub total: u64,
    pub mastered: u64,
    pub learning: u64,
    pub new: u64,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total: u64,
    pub mastered: u64,
    pub learning: u64,
    pub new: u64,
    pub per_part_of_speech: Vec<PartOfSpeechStats>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FlashcardMetadata {
    Noun {
        gender: Option<String>,
        plural: Option<String>,
        suffix: Option<String>,
    },
    Verb {
        present_form: Option<String>,
        preterite_form: Option<String>,
        perfect_form: Option<String>,
        properties: Option<String>,
        noun_form: Option<String>,
    },
    AdjectiveAdverb {
        attribute: Option<String>,
        comparison_forms: Vec<String>,
    },
}

impl FlashcardResponse {
    pub fn from_models(
        entry: vocabulary_entries::Model,
        progress: Option<flashcard_progress::Model>,
    ) -> Self {
        let (status, times_seen, times_mastered, last_seen_at) = match progress {
            Some(flashcard_progress::Model {
                status,
                times_seen,
                times_mastered,
                last_seen_at,
                ..
            }) => (
                Some(status),
                times_seen,
                times_mastered,
                last_seen_at.map(|dt| dt.to_rfc3339()),
            ),
            None => (None, 0, 0, None),
        };

        let metadata = FlashcardMetadata::from_entry(&entry);

        Self {
            entry_id: entry.entry_id,
            word: entry.word,
            part_of_speech: entry.part_of_speech,
            meaning: entry.meaning,
            english: entry.english,
            examples: entry.examples,
            themes: entry.themes,
            status,
            times_seen,
            times_mastered,
            last_seen_at,
            metadata,
        }
    }
}

impl FlashcardMetadata {
    fn from_entry(entry: &vocabulary_entries::Model) -> Option<Self> {
        let extra = entry.extra.as_ref()?;
        let data = extra.as_object()?;

        match entry.part_of_speech.as_str() {
            "noun" => Some(Self::noun_from_extra(data)),
            "verb" => Some(Self::verb_from_extra(data)),
            "adjective_adverb" => Some(Self::adjective_from_extra(data)),
            _ => None,
        }
    }

    fn noun_from_extra(data: &serde_json::Map<String, JsonValue>) -> Self {
        Self::Noun {
            gender: extract_string(data, "gender"),
            plural: extract_string(data, "plural"),
            suffix: extract_string(data, "suffix"),
        }
    }

    fn verb_from_extra(data: &serde_json::Map<String, JsonValue>) -> Self {
        Self::Verb {
            present_form: extract_string(data, "present_form"),
            preterite_form: extract_string(data, "preterite_form"),
            perfect_form: extract_string(data, "perfect_form"),
            properties: extract_string(data, "properties"),
            noun_form: extract_string(data, "noun_form"),
        }
    }

    fn adjective_from_extra(data: &serde_json::Map<String, JsonValue>) -> Self {
        let comparison_delimiters = [',', ';', '\u{FF1B}'];
        let comparison_forms = match data.get("comparison_forms") {
            Some(JsonValue::String(value)) => value
                .split(&comparison_delimiters[..])
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .map(|part| part.to_string())
                .collect(),
            Some(JsonValue::Array(items)) => items
                .iter()
                .filter_map(|value| value.as_str())
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string())
                .collect(),
            _ => Vec::new(),
        };

        Self::AdjectiveAdverb {
            attribute: extract_string(data, "attribute"),
            comparison_forms,
        }
    }
}

fn extract_string(data: &serde_json::Map<String, JsonValue>, key: &str) -> Option<String> {
    data.get(key)
        .and_then(JsonValue::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
}
