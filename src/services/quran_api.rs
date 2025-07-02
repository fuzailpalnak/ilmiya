use crate::{model::quran::QuranApiResponse, utils};
use once_cell::sync::Lazy;
use reqwest::get;
use anyhow::{Result, Context};

static BASE_URL: Lazy<String> = Lazy::new(|| {
    utils::env::load_env_var("QURAN_API_BASE_URL").expect("QURAN_API_BASE_URL must be set in .env")
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
