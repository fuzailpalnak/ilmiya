use anyhow::{bail, Context, Result};
use reqwest::Client;
use crate::{model};


pub async fn handle_llm_response(
    api_url: &str,
    prompt: String,
    n_guesses: u32,
) -> Result<String> {
    let client = Client::new();
    let request_body = model::llm::RequestBody::new(prompt, n_guesses, 0.7);

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request to LLM API")?;

    let status = response.status();
    let body = response.text().await.context("Failed to read LLM response body")?;

    if !status.is_success() {
        bail!("LLM API Error: {} - {}", status, body);
    }

    let api_response: model::llm::ApiResponse =
        serde_json::from_str(&body).context("Failed to parse LLM API JSON")?;

    let output = api_response
        .candidates
        .as_ref()
        .and_then(|c| c.first())
        .and_then(|c| c.content.as_ref())
        .and_then(|c| c.parts.first())
        .map(|part| part.text.clone())
        .ok_or_else(|| anyhow::anyhow!("No valid text in LLM response"))?;

    Ok(output)
}




