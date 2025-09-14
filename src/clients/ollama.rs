use anyhow::Result;
use ollama_rs::Ollama;
use ollama_rs::generation::chat::ChatMessage;

#[derive(Debug)]
pub struct OllamaClient {
    ollama: Ollama,
    model: String,
}

impl OllamaClient {
    pub fn new(model: String, host: Option<String>, port: Option<u16>) -> Self {
        let ollama = match (host, port) {
            (Some(h), Some(p)) => Ollama::new(h, p),
            (Some(h), None) => Ollama::new(h, 11434),
            (None, Some(p)) => Ollama::new("http://localhost".to_string(), p),
            (None, None) => Ollama::default(),
        };
        Self { ollama, model }
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let request = ollama_rs::generation::chat::request::ChatMessageRequest::new(
            self.model.clone(),
            messages,
        );
        let response = self.ollama.send_chat_messages(request).await?;
        Ok(response.message.content.trim().to_string())
    }
}
