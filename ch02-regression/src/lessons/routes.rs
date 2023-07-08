use actix_web::web::{self, get};

use crate::lessons::l2_prepare_data::get_lesson_2;

use crate::lessons::index::get_index;

pub fn lesson_routes(config: &mut web::ServiceConfig) {
  config
    .service(web::scope("/").route("", get().to(get_index)))
    .service(web::scope("/lesson-2").route("", get().to(get_lesson_2)));
}