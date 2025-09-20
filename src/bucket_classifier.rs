use crate::clients::{GeminiClient, GoogleSearchClient, OllamaClient};
use crate::config::settings::Config;
use anyhow::Result;

const BUCKET_OTHER: &str = "Other";

#[derive(Debug)]
pub struct BucketClassifier {
    pub gemini_client: Option<GeminiClient>,
    pub ollama_client: Option<OllamaClient>,
    search_client: Option<GoogleSearchClient>,
    pub buckets: Vec<String>,
    city: Option<String>,
}

impl BucketClassifier {
    pub fn new(model: Option<String>, config: &Config) -> Self {
        let gemini_client = if let (Some(api_key), Some(model_name)) =
            (&config.gemini.api_key, &config.gemini.model)
        {
            Some(GeminiClient::new(api_key.clone(), model_name.clone()))
        } else {
            None
        };

        let ollama_client = if let Some(model_name) = model {
            Some(OllamaClient::new(
                model_name,
                config.ollama.host.clone(),
                config.ollama.port,
            ))
        } else {
            None
        };

        let search_client = if let (Some(api_key), Some(engine_id)) = (
            &config.google_search.api_key,
            &config.google_search.engine_id,
        ) {
            Some(GoogleSearchClient::new(api_key.clone(), engine_id.clone()))
        } else {
            None
        };

        let buckets = config.buckets.categories.clone();

        Self {
            gemini_client,
            ollama_client,
            search_client,
            buckets,
            city: config.city.clone(),
        }
    }

    pub async fn generate_search_query(&self, description: &str) -> Result<String> {
        let base_query = format!("what is {} business", description.to_lowercase());

        if let Some(city) = &self.city {
            Ok(format!("{} in {}", base_query, city.to_lowercase()))
        } else {
            Ok(base_query)
        }
    }

    pub async fn search(&self, query: &str) -> Result<String> {
        let search_client = self
            .search_client
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Google Search client not available"))?;
        search_client.search(query).await
    }

    pub fn find_best_bucket_match(&self, response: &str) -> Result<String> {
        let response_lower = response.to_lowercase();

        for bucket in &self.buckets {
            if response_lower.contains(&bucket.to_lowercase()) {
                return Ok(bucket.clone());
            }
        }

        let response_words: Vec<&str> = response_lower.split_whitespace().collect();
        for bucket in &self.buckets {
            let bucket_lower = bucket.to_lowercase();
            let bucket_words: Vec<&str> = bucket_lower.split_whitespace().collect();
            for response_word in &response_words {
                for bucket_word in &bucket_words {
                    if response_word == bucket_word {
                        return Ok(bucket.clone());
                    }
                }
            }
        }

        Ok(BUCKET_OTHER.to_string())
    }

    pub async fn classify_transaction_with_fallback(
        &self,
        transaction: &crate::clients::investec::models::Transaction,
    ) -> Result<String> {
        if self.gemini_client.is_some() {
            if let Ok(result) = self.try_gemini_with_search(transaction).await {
                return Ok(result);
            }
        }

        if self.ollama_client.is_some() && self.search_client.is_some() {
            println!("Trying Ollama with search");
            if let Ok(result) = self.try_ollama_with_search(transaction).await {
                return Ok(result);
            }
        }

        if self.ollama_client.is_some() {
            if let Ok(result) = self.try_ollama_only(transaction).await {
                return Ok(result);
            }
        }

        Ok(BUCKET_OTHER.to_string())
    }

    async fn try_gemini_with_search(
        &self,
        transaction: &crate::clients::investec::models::Transaction,
    ) -> Result<String> {
        let prompt = format!(
            "Classify this transaction into one of these buckets: {}\n\n\
            Transaction: {}\n\
            Amount: {:.2}\n\n\
            IMPORTANT: You MUST perform a web search to get current information about what this transaction represents.\n\
            Use the search results to understand the business/merchant and make an informed classification.\n\
            Return only the bucket name, nothing else:",
            self.buckets.join(", "),
            transaction.description,
            transaction.amount
        );

        let gemini_client = self
            .gemini_client
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Gemini client not available"))?;

        // Gemini uses its built-in Google Search tool - no external API needed
        let response = gemini_client.generate_text_with_search(&prompt).await?;

        println!("Gemini with built-in search response: {}", response);

        self.process_classification_response(&response, "Gemini with built-in search")
    }

    async fn try_ollama_with_search(
        &self,
        transaction: &crate::clients::investec::models::Transaction,
    ) -> Result<String> {
        let ollama_client = self
            .ollama_client
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Ollama client not available"))?;

        let search_query = self.generate_search_query(&transaction.description).await?;
        let search_results = self.search(&search_query).await?;

        let prompt = self.create_classification_prompt(transaction, Some(&search_results));

        let messages = vec![ollama_rs::generation::chat::ChatMessage::user(prompt)];
        let response = ollama_client.chat(messages).await?;

        self.process_classification_response(&response, "Ollama + Search")
    }

    async fn try_ollama_only(
        &self,
        transaction: &crate::clients::investec::models::Transaction,
    ) -> Result<String> {
        let ollama_client = self
            .ollama_client
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Ollama client not available"))?;

        let prompt = format!(
            "Classify this transaction: '{}'\n\
             Amount: {:.2}\n\n\
             Buckets: {}\n\n\
             Based on the description, which bucket does this belong to?\n\
             Return only the bucket name:",
            transaction.description,
            transaction.amount,
            self.buckets.join(", ")
        );

        let messages = vec![ollama_rs::generation::chat::ChatMessage::user(prompt)];
        let response = ollama_client.chat(messages).await?;

        self.process_classification_response(&response, "Ollama only")
    }

    fn create_classification_prompt(
        &self,
        transaction: &crate::clients::investec::models::Transaction,
        search_context: Option<&str>,
    ) -> String {
        let base_prompt = format!(
            "Classify this transaction into one of these buckets: {}\n\n\
            Transaction: {}\n\
            Amount: {:.2}\n\n",
            self.buckets.join(", "),
            transaction.description,
            transaction.amount
        );

        match search_context {
            Some(context) => format!(
                "{}Search results for context:\n{}\n\n\
                Based on the transaction description and search results, which bucket does this belong to?\n\
                Return only the bucket name, nothing else:",
                base_prompt, context
            ),
            None => format!(
                "{}IMPORTANT: You MUST perform a web search to get current information about what this transaction represents.\n\
                Use the search results to understand the business/merchant and make an informed classification.\n\
                Return only the bucket name, nothing else:",
                base_prompt
            ),
        }
    }

    fn process_classification_response(
        &self,
        response: &str,
        strategy_name: &str,
    ) -> Result<String> {
        match self.find_best_bucket_match(response) {
            Ok(bucket) => {
                if bucket != BUCKET_OTHER.to_string() {
                    println!("      → Bucket: {} (via {})", bucket, strategy_name);
                    Ok(bucket)
                } else {
                    Err(anyhow::anyhow!("Classification returned 'Other' bucket"))
                }
            }
            Err(e) => {
                println!("      → {} parsing failed: {}", strategy_name, e);
                Err(anyhow::anyhow!("{} failed: {}", strategy_name, e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::settings::{
        BucketsConfig, Config, DatabaseConfig, GeminiConfig, GoogleSearchConfig, InvestecConfig,
        OllamaConfig,
    };

    fn create_test_config() -> Config {
        Config {
            investec: InvestecConfig {
                x_api_key: "test".to_string(),
                client_id: "test".to_string(),
                client_secret: "test".to_string(),
            },
            google_search: GoogleSearchConfig {
                api_key: Some("test".to_string()),
                engine_id: Some("test".to_string()),
            },
            gemini: GeminiConfig {
                api_key: Some("test".to_string()),
                model: Some("test".to_string()),
            },
            ollama: OllamaConfig {
                model: Some("test".to_string()),
                host: None,
                port: None,
            },
            city: Some("cape town".to_string()),
            database: DatabaseConfig {
                url: "sqlite://test.db".to_string(),
            },
            buckets: BucketsConfig {
                categories: vec![
                    "Food".to_string(),
                    "Transportation".to_string(),
                    "Entertainment".to_string(),
                    "Bills & Utilities".to_string(),
                    "Healthcare".to_string(),
                    "Income".to_string(),
                    "Transfers".to_string(),
                    "Other".to_string(),
                ],
            },
        }
    }

    #[test]
    fn test_find_best_bucket_match_exact() {
        let config = create_test_config();
        let classifier = BucketClassifier::new(Some("test".to_string()), &config);

        // Test exact match
        assert_eq!(classifier.find_best_bucket_match("Food").unwrap(), "Food");
    }

    #[test]
    fn test_find_best_bucket_match_partial() {
        let config = create_test_config();
        let classifier = BucketClassifier::new(Some("test".to_string()), &config);

        // Test partial match
        assert_eq!(
            classifier
                .find_best_bucket_match("This is a food transaction")
                .unwrap(),
            "Food"
        );
    }

    #[test]
    fn test_find_best_bucket_match_default() {
        let config = create_test_config();
        let classifier = BucketClassifier::new(Some("test".to_string()), &config);

        // Test default case
        assert_eq!(
            classifier
                .find_best_bucket_match("Unknown category")
                .unwrap(),
            "Other"
        );
    }
}
