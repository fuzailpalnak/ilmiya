pub mod create;
pub mod delete;
pub mod edit;
pub mod fetch;
pub mod mcq;
pub mod quran;
use actix_web::{web, Scope};

pub fn exam_routes() -> Scope {
    web::scope("/exam")
        .service(web::resource("/create").route(web::post().to(create::create_exam)))
        .service(web::resource("/edit").route(web::put().to(edit::edit_exam)))
        .service(web::resource("/{exam_id}").route(web::get().to(fetch::fetch_exam)))
        .service(web::resource("/delete/{exam_id}").route(web::delete().to(delete::delete_exam)))
}

pub fn mcq_routes() -> Scope {
    web::scope("/mcq")
        .service(web::resource("/options/similar").route(web::post().to(mcq::generate_mcq_options_for_quranic_verses)))
        .service(web::resource("/options/context").route(web::post().to(mcq::generate_mcq_options_from_context)))
}

pub fn quran_routes() -> Scope {
    web::scope("/quran")
        .service(web::resource("/verse").route(web::post().to(quran::get_quran_verse_indo_pak_script)))
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(exam_routes());
    cfg.service(mcq_routes());
    cfg.service(quran_routes());
}

