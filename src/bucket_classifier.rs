use crate::clients::google::google::GoogleSearchClient;
use crate::clients::ollama::OllamaClient;
use crate::config::settings::Config;
use anyhow::Result;

const BUCKET_FOOD: &str = "Food";
const BUCKET_TRANSPORTATION: &str = "Transportation";
const BUCKET_ENTERTAINMENT: &str = "Entertainment";
const BUCKET_BILLS_UTILITIES: &str = "Bills & Utilities";
const BUCKET_HEALTHCARE: &str = "Healthcare";
const BUCKET_INCOME: &str = "Income";
const BUCKET_TRANSFERS: &str = "Transfers";
const BUCKET_OTHER: &str = "Other";

#[derive(Debug)]
pub struct BucketClassifier {
    ollama_client: OllamaClient,
    search_client: GoogleSearchClient,
    buckets: Vec<String>,
    city: Option<String>,
}

impl BucketClassifier {
    pub fn new(model: String, config: &Config) -> Self {
        let ollama_client = OllamaClient::new(model);

        let search_client = GoogleSearchClient::new(
            config.google_search_api_key.clone(),
            config.google_search_engine_id.clone(),
        );
        let buckets = vec![
            BUCKET_FOOD.to_string(),
            BUCKET_TRANSPORTATION.to_string(),
            BUCKET_ENTERTAINMENT.to_string(),
            BUCKET_BILLS_UTILITIES.to_string(),
            BUCKET_HEALTHCARE.to_string(),
            BUCKET_INCOME.to_string(),
            BUCKET_TRANSFERS.to_string(),
            BUCKET_OTHER.to_string(),
        ];

        Self {
            ollama_client,
            search_client,
            buckets,
            city: config.city.clone(),
        }
    }

    async fn generate_search_query(&self, description: &str) -> Result<String> {
        let base_query = format!("what is {} business", description.to_lowercase());

        if let Some(city) = &self.city {
            Ok(format!("{} in {}", base_query, city.to_lowercase()))
        } else {
            Ok(base_query)
        }
    }

    async fn search(&self, query: &str) -> Result<String> {
        self.search_client.search(query).await
    }

    pub async fn classify_with_search(&self, description: &str, amount: f64) -> Result<String> {
        let search_query = self.generate_search_query(description).await?;
        println!("  Search query: {}", search_query);

        let search_results = self.search(&search_query).await?;
        println!(
            "Search results: {}",
            search_results.lines().next().unwrap_or("No results")
        );

        println!("search results: {}", search_results);

        let prompt = format!(
            "Classify this transaction: '{}'\n\
             Amount: {:.2}\n\n\
             Search results: {}\n\n\
             Buckets: {}\n\n\
             Based on the description and search results, which bucket does this belong to?\n\
             Return only the bucket name:",
            description,
            amount,
            search_results.lines().next().unwrap_or("No search results"),
            self.buckets.join(", ")
        );

        let messages = vec![ollama_rs::generation::chat::ChatMessage::user(prompt)];
        let bucket_response = self.ollama_client.chat(messages).await?;

        self.find_best_bucket_match(&bucket_response)
    }

    pub async fn classify_transaction_without_search(
        &self,
        description: &str,
        amount: f64,
    ) -> Result<String> {
        let prompt = format!(
            "You are a financial transaction classifier. Your task is to categorize transactions into the most appropriate bucket.\n\n\
             Available buckets: {}\n\n\
             Transaction details:\n\
             - Description: {}\n\
             - Amount: {:.2}\n\n\
             Examples:\n\
             - 'STARBUCKS COFFEE' → Food\n\
             - 'WOOLWORTHS' → Food\n\
             - 'UBER TRIP' → Transportation\n\
             - 'NETFLIX SUBSCRIPTION' → Entertainment\n\
             - 'AMAZON PURCHASE' → Entertainment\n\
             - 'ELECTRICITY BILL' → Bills & Utilities\n\
             - 'DOCTOR VISIT' → Healthcare\n\
             - 'FLIGHT TICKET' → Transportation\n\
             - 'SALARY DEPOSIT' → Income\n\
             - 'BANK TRANSFER' → Transfers\n\n\
             Based on the description and amount pattern, classify this transaction.\n\
             Return ONLY the bucket name, nothing else.",
            self.buckets.join(", "),
            description,
            amount
        );

        let messages = vec![ollama_rs::generation::chat::ChatMessage::user(prompt)];
        let bucket_response = self.ollama_client.chat(messages).await?;

        self.find_best_bucket_match(&bucket_response)
    }

    fn find_best_bucket_match(&self, response: &str) -> Result<String> {
        let response_lower = response.to_lowercase();

        // If the llm behaved as expected, then the response should contain the bucket name
        for bucket in &self.buckets {
            if response_lower.contains(&bucket.to_lowercase()) {
                return Ok(bucket.clone());
            }
        }

        // Try partial word matches - check if any word in the response matches
        // any word in a bucket name
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::settings::Config;

    fn create_test_config() -> Config {
        Config {
            x_api_key: "test".to_string(),
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            google_search_api_key: "test".to_string(),
            google_search_engine_id: "test".to_string(),
            ollama_model: "test".to_string(),
            city: Some("cape town".to_string()),
        }
    }

    #[test]
    fn test_find_best_bucket_match_exact() {
        let config = create_test_config();
        let classifier = BucketClassifier::new("test".to_string(), &config);

        // Test exact match
        assert_eq!(
            classifier.find_best_bucket_match(BUCKET_FOOD).unwrap(),
            BUCKET_FOOD
        );
    }

    #[test]
    fn test_find_best_bucket_match_partial() {
        let config = create_test_config();
        let classifier = BucketClassifier::new("test".to_string(), &config);

        // Test partial match
        assert_eq!(
            classifier
                .find_best_bucket_match("This is a food transaction")
                .unwrap(),
            BUCKET_FOOD
        );
    }

    #[test]
    fn test_find_best_bucket_match_default() {
        let config = create_test_config();
        let classifier = BucketClassifier::new("test".to_string(), &config);

        // Test default case
        assert_eq!(
            classifier
                .find_best_bucket_match("Unknown category")
                .unwrap(),
            BUCKET_OTHER
        );
    }
}
