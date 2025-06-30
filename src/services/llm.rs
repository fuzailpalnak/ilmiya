use anyhow::{bail, Context, Result};
use reqwest::Client;
use crate::model;

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
    api_url: &str,
    prompt: String,
    n_guesses: u32,
) -> Result<String> {
    let client = Client::new();
    let request_body = model::request::LLMRequest::new(prompt.to_owned(), n_guesses, 0.7);

    let response = client
        .post(api_url)
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
    let api_response: model::response::LLMResponse =
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