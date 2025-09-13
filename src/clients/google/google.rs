use anyhow::Result;
use reqwest::Client;

use super::models::GoogleSearchResponse;

#[derive(Debug)]
pub struct GoogleSearchClient {
    client: Client,
    api_key: String,
    search_engine_id: String,
}

impl GoogleSearchClient {
    pub fn new(api_key: String, search_engine_id: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            search_engine_id,
        }
    }

    pub async fn search(&self, query: &str) -> Result<String> {
        let search_url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num=3",
            self.api_key,
            self.search_engine_id,
            urlencoding::encode(query)
        );

        let response_text = self.client.get(&search_url).send().await?.text().await?;

        match serde_json::from_str::<GoogleSearchResponse>(&response_text) {
            Ok(response) => {
                let formatted_results = if let Some(items) = response.items {
                    self.format_search_results(items)
                } else {
                    String::new()
                };

                if formatted_results.is_empty() {
                    Ok(format!("No relevant information found for: {}", query))
                } else {
                    Ok(formatted_results)
                }
            }
            Err(_) => {
                println!(
                    "  Search API returned: {}",
                    response_text.lines().next().unwrap_or("Unknown response")
                );
                Ok(format!("Search completed for: {}", query))
            }
        }
    }

    fn format_search_results(&self, items: Vec<super::models::SearchItem>) -> String {
        let mut context = String::new();

        for (index, item) in items.into_iter().enumerate() {
            // Format the title with numbering
            context.push_str(&format!("{}. {}\n", index + 1, item.title));

            // Add snippet if available
            if let Some(snippet) = item.snippet {
                context.push_str(&format!("   {}\n", snippet));
            }

            // Always add the link
            context.push_str(&format!("   {}\n\n", item.link));
        }

        context
    }
}
