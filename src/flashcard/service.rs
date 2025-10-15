use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, FromQueryResult,
    PaginatorTrait, QueryFilter, QueryOrder, Statement, TransactionTrait, Value,
    prelude::DateTimeWithTimeZone,
};

use crate::{
    entity::{flashcard_progress, flashcard_reviews, vocabulary_entries},
    error::AppError,
    flashcard::dto::{
        FlashcardResponse, NextCardQuery, PartOfSpeechStats, ReviewRequest, StatsResponse,
    },
    state::SharedState,
};

pub struct FlashcardService {
    state: SharedState,
}

impl FlashcardService {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    fn db(&self) -> &DatabaseConnection {
        &self.state.db
    }

    pub async fn get_next_card(
        &self,
        params: NextCardQuery,
    ) -> Result<Option<FlashcardResponse>, AppError> {
        let db = self.db();
        let status_condition = build_status_condition(params.status.as_deref())?;

        let mut select =
            vocabulary_entries::Entity::find().find_also_related(flashcard_progress::Entity);
        select = select.filter(status_condition);

        if let Some(part) = params.part_of_speech.as_deref() {
            let normalized = normalize_part_of_speech(part)?;
            select = select.filter(vocabulary_entries::Column::PartOfSpeech.eq(normalized));
        }

        let result = select
            .order_by_asc(flashcard_progress::Column::TimesSeen)
            .order_by_asc(vocabulary_entries::Column::SourceCreatedTime)
            .order_by_asc(vocabulary_entries::Column::EntryId)
            .one(db)
            .await?;

        Ok(result.map(|(entry, progress)| FlashcardResponse::from_models(entry, progress)))
    }

    pub async fn record_review(&self, entry_id: i32, req: ReviewRequest) -> Result<(), AppError> {
        let status = normalize_status(&req.result)?;
        let status_str = status.to_string();
        let now_utc = Utc::now();
        let now: DateTimeWithTimeZone = now_utc.into();
        let db = self.db();
        let Some(_entry) = vocabulary_entries::Entity::find_by_id(entry_id)
            .one(db)
            .await?
        else {
            return Err(AppError::NotFound);
        };

        let txn = db.begin().await?;

        let existing = flashcard_progress::Entity::find()
            .filter(flashcard_progress::Column::EntryId.eq(entry_id))
            .one(&txn)
            .await?;

        match existing {
            Some(model) => {
                let times_seen = model.times_seen + 1;
                let times_mastered = if status == STATUS_MASTERED {
                    model.times_mastered + 1
                } else {
                    model.times_mastered
                };
                let mut active: flashcard_progress::ActiveModel = model.into();
                active.status = Set(status_str.clone());
                active.times_seen = Set(times_seen);
                active.times_mastered = Set(times_mastered);
                active.last_seen_at = Set(Some(now.clone()));
                active.updated_at = Set(now.clone());
                active.update(&txn).await?;
            }
            None => {
                let times_mastered = if status == STATUS_MASTERED { 1 } else { 0 };
                let active = flashcard_progress::ActiveModel {
                    progress_id: NotSet,
                    entry_id: Set(entry_id),
                    status: Set(status_str.clone()),
                    times_seen: Set(1),
                    times_mastered: Set(times_mastered),
                    last_seen_at: Set(Some(now.clone())),
                    created_at: Set(now.clone()),
                    updated_at: Set(now.clone()),
                };
                active.insert(&txn).await?;
            }
        }

        let review = flashcard_reviews::ActiveModel {
            review_id: NotSet,
            entry_id: Set(entry_id),
            result: Set(status_str),
            notes: Set(req.notes),
            reviewed_at: Set(now.clone()),
        };
        review.insert(&txn).await?;

        txn.commit().await?;
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<StatsResponse, AppError> {
        let db = self.db();
        let total = vocabulary_entries::Entity::find().count(db).await?;
        let total_progress = flashcard_progress::Entity::find().count(db).await?;
        let mastered = flashcard_progress::Entity::find()
            .filter(flashcard_progress::Column::Status.eq(STATUS_MASTERED))
            .count(db)
            .await?;
        let learning = flashcard_progress::Entity::find()
            .filter(flashcard_progress::Column::Status.ne(STATUS_MASTERED))
            .count(db)
            .await?;
        let new = total.saturating_sub(total_progress);

        let backend = db.get_database_backend();
        let stats_rows = PartStatsRow::find_by_statement(Statement::from_sql_and_values(
            backend,
            r#"
                SELECT ve.part_of_speech,
                       COUNT(*) AS total,
                       COALESCE(SUM(CASE WHEN fp.status = 'mastered' THEN 1 ELSE 0 END), 0) AS mastered,
                       COALESCE(SUM(CASE WHEN fp.status IS NOT NULL AND fp.status <> 'mastered' THEN 1 ELSE 0 END), 0) AS learning,
                       COALESCE(SUM(CASE WHEN fp.entry_id IS NULL THEN 1 ELSE 0 END), 0) AS new
                FROM vocabulary_entries ve
                LEFT JOIN flashcard_progress fp ON fp.entry_id = ve.entry_id
                GROUP BY ve.part_of_speech
                ORDER BY ve.part_of_speech
            "#,
            Vec::<Value>::new(),
        ))
        .all(db)
        .await?;

        let per_part_of_speech = stats_rows
            .into_iter()
            .map(|row| PartOfSpeechStats {
                part_of_speech: row.part_of_speech,
                total: row.total as u64,
                mastered: row.mastered as u64,
                learning: row.learning as u64,
                new: row.new as u64,
            })
            .collect();

        Ok(StatsResponse {
            total,
            mastered,
            learning,
            new,
            per_part_of_speech,
        })
    }
}

impl From<SharedState> for FlashcardService {
    fn from(state: SharedState) -> Self {
        Self::new(state)
    }
}

const STATUS_MASTERED: &str = "mastered";
const STATUS_LEARNING: &str = "learning";

fn build_status_condition(status: Option<&str>) -> Result<Condition, AppError> {
    let condition = match status {
        None => Condition::any()
            .add(flashcard_progress::Column::Status.is_null())
            .add(flashcard_progress::Column::Status.ne(STATUS_MASTERED)),
        Some(value) => {
            let normalized = value.trim().to_lowercase();
            match normalized.as_str() {
                "new" => Condition::all().add(flashcard_progress::Column::Status.is_null()),
                "mastered" => {
                    Condition::all().add(flashcard_progress::Column::Status.eq(STATUS_MASTERED))
                }
                "learning" => Condition::all()
                    .add(flashcard_progress::Column::Status.ne(STATUS_MASTERED))
                    .add(flashcard_progress::Column::Status.is_not_null()),
                other => {
                    return Err(AppError::Validation(format!(
                        "unsupported filter status '{}'",
                        other
                    )));
                }
            }
        }
    };

    Ok(condition)
}

fn normalize_status(input: &str) -> Result<&'static str, AppError> {
    let normalized = input.trim().to_lowercase();
    match normalized.as_str() {
        "mastered" | "m" => Ok(STATUS_MASTERED),
        "learning" | "l" | "again" | "review" => Ok(STATUS_LEARNING),
        other => Err(AppError::Validation(format!(
            "unsupported review status '{}'",
            other
        ))),
    }
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

#[derive(Debug, FromQueryResult)]
struct PartStatsRow {
    part_of_speech: String,
    total: i64,
    mastered: i64,
    learning: i64,
    new: i64,
}
