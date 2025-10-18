use std::env;

const DEFAULT_ADDR: &str = "127.0.0.1:8080";

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_addr: String,
    pub auth_secret: String,
    pub auth_cookie_name: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let database_url = env::var("DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URI"))?;
        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
        let auth_secret = env::var("AUTH_SECRET").unwrap_or_else(|_| "dev_secret_change_me".to_string());
        let auth_cookie_name = env::var("AUTH_COOKIE_NAME").unwrap_or_else(|_| "ba_session".to_string());
        Ok(Self {
            database_url,
            server_addr,
            auth_secret,
            auth_cookie_name,
        })
    }
}
