use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleSearchResponse {
    pub items: Option<Vec<SearchItem>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchItem {
    pub title: String,
    pub link: String,
    pub snippet: Option<String>,
    #[serde(rename = "htmlSnippet")]
    pub html_snippet: Option<String>,
    #[serde(rename = "displayLink")]
    pub display_link: Option<String>,
}
