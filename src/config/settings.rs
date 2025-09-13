use std::env;

use super::errors::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    /// API key for authentication
    pub x_api_key: String,
    /// Client ID for API access
    pub client_id: String,
    /// Client secret for API access
    pub client_secret: String,
    /// Google Search API key
    pub google_search_api_key: String,
    /// Google Custom Search Engine ID
    pub google_search_engine_id: String,
    /// Default Ollama model for AI classification
    pub ollama_model: String,
    /// City for location-specific search results (optional)
    pub city: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            x_api_key: Self::get_required_var("X_API_KEY")?,
            client_id: Self::get_required_var("CLIENT_ID")?,
            client_secret: Self::get_required_var("CLIENT_SECRET")?,
            google_search_api_key: Self::get_required_var("GOOGLE_SEARCH_API_KEY")?,
            google_search_engine_id: Self::get_required_var("GOOGLE_SEARCH_ENGINE_ID")?,
            ollama_model: Self::get_required_var("OLLAMA_MODEL")?,
            city: Self::get_optional_var("CITY"),
        })
    }

    fn get_required_var(key: &str) -> Result<String, ConfigError> {
        env::var(key).map_err(|_| ConfigError::MissingRequiredVar(key.to_string()))
    }

    fn get_optional_var(key: &str) -> Option<String> {
        env::var(key).ok()
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
