use axum::{Router, routing::{get, post}, extract::State, Json};
use axum::http::HeaderMap;
use chrono::{Utc, NaiveDate, Duration};
use sea_orm::{Statement, ConnectionTrait, FromQueryResult, Value};
use serde::{Deserialize, Serialize};

use crate::{state::SharedState, auth::current_user_from_headers, error::AppError};

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/api/v1/checkin", post(post_checkin).get(get_status))
        .with_state(state)
}

#[derive(Serialize)]
struct CheckinStatus {
    today_checked: bool,
    current_streak: u64,
    total_days: u64,
    last_date: Option<String>,
    recent: Vec<RecentDay>,
}

#[derive(Serialize)]
struct RecentDay { date: String, checked: bool }

async fn post_checkin(State(state): State<SharedState>, headers: HeaderMap) -> Result<Json<CheckinStatus>, AppError> {
    let user = current_user_from_headers(&headers, &state)?;
    let today = Utc::now().date_naive();
    let now = Utc::now();
    let backend = state.db.get_database_backend();

    // idempotent insert for today
    let _ = state.db.execute(Statement::from_sql_and_values(
        backend,
        "INSERT INTO user_checkins (user_id, day, checked_at) VALUES ($1, $2, $3) ON CONFLICT (user_id, day) DO NOTHING",
        vec![user.user_id.clone().into(), Value::from(today), now.into()],
    )).await?;

    // return status after insert
    let status = compute_status(&state, &user.user_id, today).await?;
    Ok(Json(status))
}

async fn get_status(State(state): State<SharedState>, headers: HeaderMap) -> Result<Json<CheckinStatus>, AppError> {
    let user = current_user_from_headers(&headers, &state)?;
    let today = Utc::now().date_naive();
    let status = compute_status(&state, &user.user_id, today).await?;
    Ok(Json(status))
}

async fn compute_status(state: &SharedState, user_id: &str, today: NaiveDate) -> Result<CheckinStatus, AppError> {
    let backend = state.db.get_database_backend();

    // total days
    #[derive(FromQueryResult)]
    struct CountRow { c: i64 }
    let total = CountRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        "SELECT COUNT(*) AS c FROM user_checkins WHERE user_id = $1",
        vec![user_id.into()],
    )).one(&state.db).await?.map(|r| r.c).unwrap_or(0);

    // last_date
    #[derive(FromQueryResult)]
    struct MaxRow { d: Option<NaiveDate> }
    let last_date = MaxRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        "SELECT MAX(day) AS d FROM user_checkins WHERE user_id = $1",
        vec![user_id.into()],
    )).one(&state.db).await?.and_then(|r| r.d);

    // recent N days presence
    let lookback = 30i64;
    #[derive(FromQueryResult)]
    struct DayRow { day: NaiveDate }
    let from = today - Duration::days(lookback-1);
    let recent_days = DayRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        "SELECT day FROM user_checkins WHERE user_id = $1 AND day >= $2",
        vec![user_id.into(), Value::from(from)],
    )).all(&state.db).await?;
    use std::collections::HashSet; let set: HashSet<_> = recent_days.into_iter().map(|r| r.day).collect();
    let mut recent = Vec::new();
    for i in 0..lookback { let d = today - Duration::days(lookback-1 - i); recent.push(RecentDay{ date: d.to_string(), checked: set.contains(&d) }); }

    // today checked?
    let today_checked = set.contains(&today) || last_date == Some(today);

    // compute streak up to today
    #[derive(FromQueryResult)]
    struct StreakRow { day: NaiveDate }
    let rows = StreakRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        "SELECT day FROM user_checkins WHERE user_id = $1 AND day >= $2 ORDER BY day DESC",
        vec![user_id.into(), Value::from(today - Duration::days(120))],
    )).all(&state.db).await?;
    let mut streak = 0u64; let mut expected = today;
    for r in rows { if r.day == expected { streak += 1; expected = expected - Duration::days(1); } else if r.day < expected { break; } }

    Ok(CheckinStatus {
        today_checked,
        current_streak: streak,
        total_days: total as u64,
        last_date: last_date.map(|d| d.to_string()),
        recent,
    })
}

