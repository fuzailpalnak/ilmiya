use actix_web::{web, HttpResponse, Error};
use anyhow::Result;

use crate::{model::{quran::{QuranApiRequest, QuranData}}, services::quran_api::fetch_verse};

pub async fn get_verse(
    req_body: web::Json<QuranApiRequest>
) -> Result<HttpResponse, Error> {
    let api_response = fetch_verse(req_body.surah, req_body.verse)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    if api_response.code != 200 {
        return Err(actix_web::error::ErrorNotFound("Invalid surah/verse or not found"));
    }

    match api_response.data {
        QuranData::Verse(verse) => Ok(HttpResponse::Ok().json(verse)),
        QuranData::ErrorMessage(msg) => {
            Err(actix_web::error::ErrorNotFound(msg))
        }
    }
}