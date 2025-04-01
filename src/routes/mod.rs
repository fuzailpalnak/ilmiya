pub mod exam;

use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(exam::exam_routes());
}
