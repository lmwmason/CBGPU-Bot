use crate::models::{AIMessage, AIProviderConfig};
use crate::state::Error;

pub struct AIClient {
    config: AIProviderConfig,
}

impl AIClient {
    pub fn new(config: AIProviderConfig) -> Self {
        Self { config }
    }

    pub async fn chat_completion(
        &self,
        messages: &[AIMessage],
        max_output_tokens: usize,
    ) -> Result<String, Error> {
        let api_key = std::env::var(&self.config.api_key_env)
            .map_err(|_| format!("API key env var {} not set", self.config.api_key_env))?;

        let url = format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );

        let request = serde_json::json!({
            "model": self.config.model,
            "messages": messages.iter().map(|m| serde_json::json!({
                "role": m.role,
                "content": m.content,
            })).collect::<Vec<_>>(),
            "max_tokens": max_output_tokens,
            "temperature": 0.7,
        });

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("AI API error ({}): {}", status, body).into());
        }

        let json: serde_json::Value = response.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("No content in AI response")?
            .to_string();

        Ok(content)
    }
}
