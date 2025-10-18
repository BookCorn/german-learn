use anyhow::Context as _;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
};
use axum::{routing::post, Router, response::IntoResponse};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2};
use rand_core::OsRng;
use sea_orm::{ConnectionTrait, Statement, FromQueryResult, Value};

use crate::state::SharedState;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JwtClaims {
    sub: String,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    iss: Option<String>,
    #[serde(default)]
    iat: Option<i64>,
    #[serde(default)]
    exp: Option<i64>,
}

#[derive(Debug)]
pub struct AuthError;

impl From<AuthError> for (StatusCode, &'static str) {
    fn from(_: AuthError) -> Self {
        (StatusCode::UNAUTHORIZED, "unauthorized")
    }
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let auth = headers.get(header::AUTHORIZATION)?;
    let auth = auth.to_str().ok()?;
    let prefix = "Bearer ";
    if auth.starts_with(prefix) {
        Some(&auth[prefix.len()..])
    } else {
        None
    }
}

fn find_cookie<'a>(cookie_header: &'a str, name: &str) -> Option<&'a str> {
    // naive cookie parsing; good enough here
    for kv in cookie_header.split(';') {
        let kv = kv.trim();
        if let Some((k, v)) = kv.split_once('=') {
            if k.trim() == name {
                return Some(v.trim());
            }
        }
    }
    None
}

pub fn validate_token(token: &str, secret: &str) -> anyhow::Result<CurrentUser> {
    let mut validation = Validation::new(Algorithm::HS256);
    // Do not enforce issuer/audience by default; can be tightened via env later
    let data = decode::<JwtClaims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)
        .context("jwt decode failed")?;

    Ok(CurrentUser {
        user_id: data.claims.sub,
        email: data.claims.email,
        name: data.claims.name,
    })
}

// Public helper for handlers
pub fn current_user_from_headers(headers: &HeaderMap, app: &SharedState) -> Result<CurrentUser, crate::error::AppError> {
    let cookie_name = &app.config.auth_cookie_name;
    let secret = &app.config.auth_secret;

    if let Some(token) = bearer_token(headers) {
        return validate_token(token, secret).map_err(|_| crate::error::AppError::Unauthorized);
    }
    if let Some(raw_cookie) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = raw_cookie.to_str() {
            if let Some(token) = find_cookie(cookie_str, cookie_name) {
                return validate_token(token, secret).map_err(|_| crate::error::AppError::Unauthorized);
            }
        }
    }
    Err(crate::error::AppError::Unauthorized)
}

// --- Dev login route (for local testing only) ---------------------------------
#[derive(Deserialize)]
struct DevLoginRequest {
    user_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
}

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/api/auth/dev-login", post(dev_login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", post(me))
        .with_state(state)
}

async fn dev_login(
    State(app): State<SharedState>,
    axum::Json(req): axum::Json<DevLoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    // Guarded by env
    if std::env::var("DEV_LOGIN").unwrap_or_default() != "1" {
        return Err((StatusCode::FORBIDDEN, "dev login disabled"));
    }

    let uid = req.user_id.unwrap_or_else(|| "dev-user".to_string());
    let claims = serde_json::json!({
        "sub": uid,
        "email": req.email,
        "name": req.name,
        "iss": "german_learn",
        "iat": chrono::Utc::now().timestamp(),
        "exp": chrono::Utc::now().timestamp() + 60 * 60 * 24 * 30, // 30 days
    });

    let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(app.config.auth_secret.as_bytes()))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "sign error"))?;

    let cookie = format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax",
        app.config.auth_cookie_name, token
    );

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, axum::Json(serde_json::json!({"ok": true}))))
}

#[derive(Deserialize)]
struct RegisterRequest { email: String, password: String, #[serde(default)] name: Option<String> }
#[derive(Deserialize)]
struct LoginRequest { email: String, password: String }
#[derive(Serialize)]
struct MeResponse { user_id: String, email: Option<String>, name: Option<String> }

async fn register(
    State(app): State<SharedState>,
    axum::Json(req): axum::Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let email = req.email.trim().to_lowercase();
    if email.is_empty() || req.password.len() < 6 { return Err((StatusCode::BAD_REQUEST, "invalid credentials")); }
    // hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "hash error"))?
        .to_string();
    // create user_id
    let user_id = format!("u_{}", chrono::Utc::now().timestamp_nanos());
    let name = req.name.unwrap_or_default();
    let db = &app.db;
    let backend = db.get_database_backend();
    // check exists
    #[derive(FromQueryResult)] struct C { c: i64 }
    let exists = C::find_by_statement(Statement::from_sql_and_values(
        backend, "SELECT COUNT(1) c FROM users WHERE email = $1", vec![email.clone().into()]
    )).one(db).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "db error"))?.unwrap_or(C{c:0}).c > 0;
    if exists { return Err((StatusCode::CONFLICT, "email exists")); }
    // insert
    db.execute(Statement::from_sql_and_values(
        backend,
        "INSERT INTO users (user_id, email, name, password_hash, created_at) VALUES ($1,$2,$3,$4,NOW())",
        vec![user_id.clone().into(), email.clone().into(), name.into(), hash.into()]
    )).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "db error"))?;

    // issue token
    let claims = serde_json::json!({
        "sub": user_id,
        "email": email,
        "iss": "german_learn",
        "iat": chrono::Utc::now().timestamp(),
        "exp": chrono::Utc::now().timestamp() + 60 * 60 * 24 * 30,
    });
    let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(app.config.auth_secret.as_bytes()))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "sign error"))?;
    let cookie = format!("{}={}; Path=/; HttpOnly; SameSite=Lax", app.config.auth_cookie_name, token);
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());
    Ok((headers, axum::Json(serde_json::json!({"ok": true}))))
}

async fn login(
    State(app): State<SharedState>,
    axum::Json(req): axum::Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let email = req.email.trim().to_lowercase();
    #[derive(FromQueryResult)]
    struct Row { user_id: String, password_hash: Option<String>, name: Option<String> }
    let backend = app.db.get_database_backend();
    let row = Row::find_by_statement(Statement::from_sql_and_values(
        backend,
        "SELECT user_id, password_hash, name FROM users WHERE email = $1",
        vec![email.clone().into()]
    )).one(&app.db).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "db error"))?;
    let Some(row) = row else { return Err((StatusCode::UNAUTHORIZED, "invalid login")); };
    let Some(phc) = row.password_hash else { return Err((StatusCode::UNAUTHORIZED, "invalid login")); };
    let parsed = PasswordHash::new(&phc).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "hash parse"))?;
    Argon2::default().verify_password(req.password.as_bytes(), &parsed).map_err(|_| (StatusCode::UNAUTHORIZED, "invalid login"))?;
    // token
    let claims = serde_json::json!({
        "sub": row.user_id,
        "email": email,
        "iss": "german_learn",
        "iat": chrono::Utc::now().timestamp(),
        "exp": chrono::Utc::now().timestamp() + 60 * 60 * 24 * 30,
    });
    let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(app.config.auth_secret.as_bytes()))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "sign error"))?;
    let cookie = format!("{}={}; Path=/; HttpOnly; SameSite=Lax", app.config.auth_cookie_name, token);
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());
    Ok((headers, axum::Json(serde_json::json!({"ok": true}))))
}

async fn logout(State(app): State<SharedState>) -> impl IntoResponse {
    let cookie = format!("{}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax", app.config.auth_cookie_name);
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());
    (headers, axum::Json(serde_json::json!({"ok": true})))
}

async fn me(State(app): State<SharedState>, headers: HeaderMap) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let user = crate::auth::current_user_from_headers(&headers, &app).map_err(|_| (StatusCode::UNAUTHORIZED, "unauthorized"))?;
    Ok(axum::Json(MeResponse { user_id: user.user_id, email: user.email, name: user.name }))
}
