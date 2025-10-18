use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, FromQueryResult,
    PaginatorTrait, QueryFilter, Statement, TransactionTrait, Value,
    prelude::{DateTimeWithTimeZone, Json},
};

use crate::{
    entity::{user_flashcard_progress, user_flashcard_reviews, vocabulary_entries},
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
        user_id: &str,
        params: NextCardQuery,
    ) -> Result<Option<FlashcardResponse>, AppError> {
        let db = self.db();
        let part_filter = match params.part_of_speech.as_deref() {
            Some(part) => Some(normalize_part_of_speech(part)?),
            None => None,
        };

        // Build SQL with left join by user on (entry_id AND user_id)
        let mut sql = String::from(
            r#"
            SELECT 
                ve.entry_id, ve.word, ve.part_of_speech, ve.user_owner, ve.english, ve.meaning,
                ve.examples, ve.themes, ve.source_table, ve.source_created_time, ve.extra,
                ufp.progress_id as ufp_progress_id, ufp.user_id as ufp_user_id,
                ufp.status as ufp_status, ufp.times_seen as ufp_times_seen,
                ufp.times_mastered as ufp_times_mastered, ufp.last_seen_at as ufp_last_seen_at,
                ufp.created_at as ufp_created_at, ufp.updated_at as ufp_updated_at
            FROM vocabulary_entries ve
            LEFT JOIN user_flashcard_progress ufp
              ON ufp.entry_id = ve.entry_id AND ufp.user_id = $1
            "#,
        );

        let mut values: Vec<Value> = vec![user_id.into()];

        // owner filter: global or owned by user
        sql.push_str(" WHERE (ve.user_owner IS NULL OR ve.user_owner = $1)");
        if let Some(part) = part_filter.as_deref() {
            sql.push_str(" AND ve.part_of_speech = $2");
            values.push(part.into());
        }

        // status filter
        match params.status.as_deref() {
            None | Some("all") => {
                // no extra filter
            }
            Some(s) => match s.to_lowercase().as_str() {
                "new" => {
                    if values.len() == 1 { sql.push_str(" WHERE "); } else { sql.push_str(" AND "); }
                    sql.push_str(" ufp.entry_id IS NULL ");
                }
                "mastered" => {
                    if values.len() == 1 { sql.push_str(" WHERE "); } else { sql.push_str(" AND "); }
                    sql.push_str(" ufp.status = 'mastered' ");
                }
                "learning" => {
                    if values.len() == 1 { sql.push_str(" WHERE "); } else { sql.push_str(" AND "); }
                    sql.push_str(" (ufp.status IS NOT NULL AND ufp.status <> 'mastered') ");
                }
                other => return Err(AppError::Validation(format!("unsupported filter status '{}'", other))),
            },
        }

        sql.push_str(" ORDER BY ufp.times_seen ASC NULLS FIRST, ve.source_created_time ASC NULLS FIRST, ve.entry_id ASC LIMIT 1");

        #[derive(FromQueryResult)]
        struct Row {
            entry_id: i32,
            word: String,
            part_of_speech: String,
            user_owner: Option<String>,
            english: Option<String>,
            meaning: Option<String>,
            examples: Option<String>,
            themes: Option<String>,
            source_table: String,
            source_created_time: Option<DateTimeWithTimeZone>,
            extra: Option<Json>,
            ufp_progress_id: Option<i64>,
            ufp_user_id: Option<String>,
            ufp_status: Option<String>,
            ufp_times_seen: Option<i32>,
            ufp_times_mastered: Option<i32>,
            ufp_last_seen_at: Option<DateTimeWithTimeZone>,
            ufp_created_at: Option<DateTimeWithTimeZone>,
            ufp_updated_at: Option<DateTimeWithTimeZone>,
        }

        let backend = db.get_database_backend();
        let rows = Row::find_by_statement(Statement::from_sql_and_values(backend, &sql, values))
            .all(db)
            .await?;

        if let Some(row) = rows.into_iter().next() {
            // Build entry model
            let entry = vocabulary_entries::Model {
                entry_id: row.entry_id,
                word: row.word,
                part_of_speech: row.part_of_speech,
                user_owner: row.user_owner,
                english: row.english,
                meaning: row.meaning,
                examples: row.examples,
                themes: row.themes,
                source_table: row.source_table,
                source_created_time: row.source_created_time,
                extra: row.extra,
            };

            let progress = row.ufp_progress_id.map(|progress_id| user_flashcard_progress::Model {
                progress_id,
                user_id: row.ufp_user_id.unwrap_or_default(),
                entry_id: entry.entry_id,
                status: row.ufp_status.unwrap_or_else(|| "learning".to_string()),
                times_seen: row.ufp_times_seen.unwrap_or(0),
                times_mastered: row.ufp_times_mastered.unwrap_or(0),
                last_seen_at: row.ufp_last_seen_at,
                created_at: row.ufp_created_at.unwrap_or_else(|| Utc::now().into()),
                updated_at: row.ufp_updated_at.unwrap_or_else(|| Utc::now().into()),
            });

            Ok(Some(FlashcardResponse::from_entry_and_user_progress(entry, progress)))
        } else {
            Ok(None)
        }
    }

    pub async fn record_review(&self, user_id: &str, entry_id: i32, req: ReviewRequest) -> Result<(), AppError> {
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

        // Ensure user row exists for FK
        let backend = txn.get_database_backend();
        let _ = sea_orm::Statement::from_sql_and_values(
            backend,
            "INSERT INTO users (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING",
            vec![user_id.into()],
        );
        txn.execute(Statement::from_sql_and_values(
            backend,
            "INSERT INTO users (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING",
            vec![user_id.into()],
        )).await?;

        let existing = user_flashcard_progress::Entity::find()
            .filter(user_flashcard_progress::Column::EntryId.eq(entry_id))
            .filter(user_flashcard_progress::Column::UserId.eq(user_id.to_string()))
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
                let mut active: user_flashcard_progress::ActiveModel = model.into();
                active.status = Set(status_str.clone());
                active.times_seen = Set(times_seen);
                active.times_mastered = Set(times_mastered);
                active.last_seen_at = Set(Some(now.clone()));
                active.updated_at = Set(now.clone());
                active.update(&txn).await?;
            }
            None => {
                let times_mastered = if status == STATUS_MASTERED { 1 } else { 0 };
                let active = user_flashcard_progress::ActiveModel {
                    progress_id: NotSet,
                    user_id: Set(user_id.to_string()),
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

        let review = user_flashcard_reviews::ActiveModel {
            review_id: NotSet,
            user_id: Set(user_id.to_string()),
            entry_id: Set(entry_id),
            result: Set(status_str),
            notes: Set(req.notes),
            reviewed_at: Set(now.clone()),
        };
        review.insert(&txn).await?;

        txn.commit().await?;
        Ok(())
    }

    pub async fn get_stats(&self, user_id: &str) -> Result<StatsResponse, AppError> {
        let db = self.db();
        // total visible entries (global + owned)
        #[derive(FromQueryResult)] struct C { total: i64 }
        let backend = db.get_database_backend();
        let total = C::find_by_statement(Statement::from_sql_and_values(
            backend,
            "SELECT COUNT(*) AS total FROM vocabulary_entries ve WHERE (ve.user_owner IS NULL OR ve.user_owner = $1)",
            vec![user_id.into()],
        )).one(db).await?.map(|c| c.total as u64).unwrap_or(0);
        let backend = db.get_database_backend();
        // totals per user
        let counts = TotalCounts::find_by_statement(Statement::from_sql_and_values(
            backend,
            r#"
                SELECT
                  COALESCE(SUM(CASE WHEN ufp.status = 'mastered' THEN 1 ELSE 0 END), 0) AS mastered,
                  COALESCE(SUM(CASE WHEN ufp.status IS NOT NULL AND ufp.status <> 'mastered' THEN 1 ELSE 0 END), 0) AS learning,
                  COALESCE(SUM(CASE WHEN ufp.entry_id IS NULL THEN 1 ELSE 0 END), 0) AS new
                FROM vocabulary_entries ve
                LEFT JOIN user_flashcard_progress ufp
                  ON ufp.entry_id = ve.entry_id AND ufp.user_id = $1
            "#,
            vec![user_id.into()],
        ))
        .one(db)
        .await?
        .unwrap_or(TotalCounts { mastered: 0, learning: 0, new: 0 });

        let stats_rows = PartStatsRow::find_by_statement(Statement::from_sql_and_values(
            backend,
            r#"
                SELECT ve.part_of_speech,
                       COUNT(*) AS total,
                       COALESCE(SUM(CASE WHEN ufp.status = 'mastered' THEN 1 ELSE 0 END), 0) AS mastered,
                       COALESCE(SUM(CASE WHEN ufp.status IS NOT NULL AND ufp.status <> 'mastered' THEN 1 ELSE 0 END), 0) AS learning,
                       COALESCE(SUM(CASE WHEN ufp.entry_id IS NULL THEN 1 ELSE 0 END), 0) AS new
                FROM vocabulary_entries ve
                LEFT JOIN user_flashcard_progress ufp
                  ON ufp.entry_id = ve.entry_id AND ufp.user_id = $1
                WHERE (ve.user_owner IS NULL OR ve.user_owner = $1)
                GROUP BY ve.part_of_speech
                ORDER BY ve.part_of_speech
            "#,
            vec![user_id.into()],
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
            mastered: counts.mastered as u64,
            learning: counts.learning as u64,
            new: counts.new as u64,
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

#[derive(Debug, FromQueryResult)]
struct TotalCounts {
    mastered: i64,
    learning: i64,
    new: i64,
}
