use std::env;

use super::errors::ConfigError;

#[derive(Debug, Clone)]
pub struct InvestecConfig {
    pub x_api_key: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone)]
pub struct GoogleSearchConfig {
    pub api_key: Option<String>,
    pub engine_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeminiConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OllamaConfig {
    pub model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub investec: InvestecConfig,
    pub google_search: GoogleSearchConfig,
    pub gemini: GeminiConfig,
    pub ollama: OllamaConfig,
    pub city: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            investec: InvestecConfig {
                x_api_key: Self::get_required_var("INVESTEC_X_API_KEY")?,
                client_id: Self::get_required_var("INVESTEC_CLIENT_ID")?,
                client_secret: Self::get_required_var("INVESTEC_CLIENT_SECRET")?,
            },
            google_search: GoogleSearchConfig {
                api_key: Self::get_optional_var("GOOGLE_SEARCH_API_KEY"),
                engine_id: Self::get_optional_var("GOOGLE_SEARCH_ENGINE_ID"),
            },
            gemini: GeminiConfig {
                api_key: Self::get_optional_var("GEMINI_API_KEY"),
                model: Self::get_optional_var("GEMINI_MODEL"),
            },
            ollama: OllamaConfig {
                model: Self::get_optional_var("OLLAMA_MODEL"),
            },
            city: Self::get_optional_var("CITY"),
        })
    }

    fn get_required_var(key: &str) -> Result<String, ConfigError> {
        env::var(key).map_err(|_| ConfigError::MissingRequiredVar(key.to_string()))
    }

    fn get_optional_var(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    pub fn validate_ai_services(&self) -> Result<(), ConfigError> {
        if self.gemini.api_key.is_some() != self.gemini.model.is_some() {
            return Err(ConfigError::MissingRequiredVar(
                "Both GEMINI_API_KEY and GEMINI_MODEL must be set together, or neither".to_string(),
            ));
        }

        if self.google_search.api_key.is_some() != self.google_search.engine_id.is_some() {
            return Err(ConfigError::MissingRequiredVar(
                "Both GOOGLE_SEARCH_API_KEY and GOOGLE_SEARCH_ENGINE_ID must be set together, or neither".to_string(),
            ));
        }

        let has_gemini = self.gemini.api_key.is_some();
        let has_ollama = self.ollama.model.is_some();

        if !has_gemini && !has_ollama {
            return Err(ConfigError::MissingRequiredVar(
                "Either GEMINI_API_KEY+GEMINI_MODEL or OLLAMA_MODEL must be set".to_string(),
            ));
        }

        Ok(())
    }

    pub fn is_ollama_available(&self) -> bool {
        self.ollama.model.is_some()
    }

    pub fn is_gemini_available(&self) -> bool {
        self.gemini.api_key.is_some()
    }

    pub fn is_google_search_available(&self) -> bool {
        self.google_search.api_key.is_some()
    }
}

pub fn load_config() -> Config {
    match Config::from_env() {
        Ok(config) => {
            if let Err(e) = config.validate_ai_services() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            config
        }
        Err(_) => {
            eprintln!("Error: Missing required configuration");
            std::process::exit(1);
        }
    }
}
