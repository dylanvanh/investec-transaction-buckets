use std::env;

use super::error::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    /// API key for authentication
    pub x_api_key: String,
    /// Client ID for API access
    pub client_id: String,
    /// Client secret for API access
    pub client_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            x_api_key: Self::get_required_var("X_API_KEY")?,
            client_id: Self::get_required_var("CLIENT_ID")?,
            client_secret: Self::get_required_var("CLIENT_SECRET")?,
        })
    }

    fn get_required_var(key: &str) -> Result<String, ConfigError> {
        env::var(key).map_err(|_| ConfigError::MissingRequiredVar(key.to_string()))
    }
}

pub fn load_config() -> Config {
    match Config::from_env() {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Missing required configuration");
            eprintln!("Set environment variables or create .cargo/config.toml");
            std::process::exit(1);
        }
    }
}
