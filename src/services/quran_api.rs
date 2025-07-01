use crate::model::quran::QuranApiResponse;
use once_cell::sync::Lazy;
use reqwest::get;
use std::env;
use anyhow::{Result, Context};

static BASE_URL: Lazy<String> = Lazy::new(|| {
    env::var("QURAN_API_BASE_URL").expect("QURAN_API_BASE_URL must be set in .env")
});

pub async fn fetch_verse(surah: u32, ayah: u32) -> Result<QuranApiResponse> {
    let url = format!("{}/{}:{}", *BASE_URL, surah, ayah);
    
    let response = get(&url)
        .await
        .context("Failed to send request to Al-Quran Cloud API")?;

    let quran_data = response
        .json::<QuranApiResponse>()
        .await
        .context("Failed to deserialize response from Al-Quran Cloud API")?;

    Ok(quran_data)
}
