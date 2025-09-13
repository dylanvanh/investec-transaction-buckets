use anyhow::Result;
use ollama_rs::Ollama;
use ollama_rs::generation::chat::ChatMessage;

#[derive(Debug)]
pub struct OllamaClient {
    ollama: Ollama,
    model: String,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        let ollama = Ollama::default();
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

    pub async fn is_available(&self) -> bool {
        match self.ollama.list_local_models().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self::new("tinyllama:latest".to_string())
    }
}
