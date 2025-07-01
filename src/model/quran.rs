use serde::{self, Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub fn deserialize_data<'de, D>(deserializer: D) -> Result<QuranData, D::Error>
where
    D: Deserializer<'de>,
{
    let val: Value = Value::deserialize(deserializer)?;

    match val {
        Value::Object(_) => {
            let verse: VerseData =
                serde_json::from_value(val).map_err(serde::de::Error::custom)?;
            Ok(QuranData::Verse(verse))
        }
        Value::String(s) => Ok(QuranData::ErrorMessage(s)),
        _ => Err(serde::de::Error::custom("Unexpected value for QuranData")),
    }
}

#[derive(Deserialize, Debug)]
pub struct QuranApiResponse {
    pub code: u32,
    pub status: String,

    #[serde(deserialize_with = "deserialize_data")]
    pub data: QuranData,
}

#[derive(Debug)]
pub enum QuranData {
    Verse(VerseData),
    ErrorMessage(String),
}


#[derive(Deserialize, Debug, Serialize)]
pub struct VerseData {
    pub number: u32,
    pub text: String,
    pub edition: Edition,
    pub surah: Surah,
    pub numberInSurah: u32,
    pub juz: u32,
    pub manzil: u32,
    pub page: u32,
    pub ruku: u32,
    pub hizbQuarter: u32,
    pub sajda: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Surah {
    pub number: u32,
    pub name: String,
    pub englishName: String,
    pub englishNameTranslation: String,
    pub numberOfAyahs: u32,
    pub revelationType: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Edition {
    pub identifier: String,
    pub language: String,
    pub name: String,
    pub englishName: String,
    pub format: String,
    #[serde(rename = "type")]
    pub edition_type: String,
    pub direction: String,
}

#[derive(Debug, Deserialize)]
pub struct QuranApiRequest {
    pub surah: u32,
    pub verse: u32,
}
