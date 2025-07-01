use anyhow::{Context, Result};
use dotenv::from_path;
use std::env;

/// Loads a specific environment variable after loading a `.env` file.
///
/// Automatically falls back to `.env` if `ENV_PATH` is not set.
///
pub fn load_env_var(key: &str) -> Result<String> {
    let env_path = env::var("ENV_PATH").unwrap_or_else(|_| ".env".to_string());

    from_path(&env_path)
        .with_context(|| format!("Failed to load .env file from path: {}", env_path))?;

    env::var(key).with_context(|| format!("Environment variable `{}` not found", key))
}
