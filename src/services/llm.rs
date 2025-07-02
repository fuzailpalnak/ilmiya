use anyhow::{bail, Context, Result};
use reqwest::Client;
use crate::{model, utils};
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct UrlBuilder {
    base_url: String,
    model_name: String,
    api_key: String,
}

impl UrlBuilder {
    fn build() -> Result<Self> {
        let base_url = utils::env::load_env_var("TEXT_GENERATION_URL")
            .context("Failed to load TEXT_GENERATION_URL environment variable")?;

        let model_name = utils::env::load_env_var("TEXT_GENERATION_MODEL")
            .context("Failed to load TEXT_GENERATION_MODEL environment variable")?;

        let api_key = utils::env::load_env_var("TEXT_GENERATION_API_KEY")
            .context("Failed to load TEXT_GENERATION_API_KEY environment variable")?;

        Ok(Self {
            base_url,
            model_name,
            api_key,
        })
    }

    pub fn get_url(&self) -> String {
        format!(
            "{}/{}:generateContent?key={}",
            self.base_url, self.model_name, self.api_key
        )
    }
}

// ✅ Lazily initialized global static instance
pub static URL_BUILDER: Lazy<UrlBuilder> = Lazy::new(|| {
    UrlBuilder::build().expect("Failed to build UrlBuilder from environment")
});


/// Sends a prompt to the LLM API and returns the generated text output.
///
/// # Arguments
/// * `api_url` - The LLM API endpoint.
/// * `prompt` - The prompt string to send.
/// * `n_guesses` - Number of guesses/options to request.
///
/// # Errors
/// Returns an error if the request fails, the API returns an error, or the response cannot be parsed.
pub async fn send_prompt_to_llm(
    prompt: String,
    n_guesses: u32,
) -> Result<String> {
    let url = URL_BUILDER.get_url();
    let client = Client::new();
    let request_body = model::llm::LLMRequest::new(prompt.to_owned(), n_guesses, 0.7);

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Uncomment and configure logging as needed
    // log::debug!("LLM API response: {:?}", response);

    let status = response.status();
    let body = response.text().await.context("Failed to read LLM response body")?;

    if !status.is_success() {
        bail!("LLM API Error: {} - {}", status, body);
    }

    parse_llm_response_text(&body)
}

/// Parses the LLM API JSON response and extracts the generated text.
fn parse_llm_response_text(body: &str) -> Result<String> {
    let api_response: model::llm::LLMResponse =
        serde_json::from_str(body).context("Failed to parse LLM API JSON")?;

    api_response
        .candidates
        .as_ref()
        .and_then(|c| c.first())
        .and_then(|c| c.content.as_ref())
        .and_then(|c| c.parts.first())
        .map(|part| part.text.clone())
        .ok_or_else(|| anyhow::anyhow!("No valid text in LLM response"))
}