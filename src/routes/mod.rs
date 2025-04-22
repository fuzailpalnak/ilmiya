pub mod create;
pub mod delete;
pub mod edit;
pub mod fetch;
use actix_web::{web, Scope};

pub fn exam_routes() -> Scope {
    actix_web::web::scope("/exam")
        .service(web::resource("/create").route(web::post().to(create::create_exam)))
        .service(web::resource("/edit").route(web::put().to(edit::edit_exam)))
        .service(web::resource("/{exam_id}").route(web::get().to(fetch::fetch_exam)))
        .service(web::resource("/delete/{exam_id}").route(web::delete().to(delete::delete_exam)))
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(exam_routes());
}
