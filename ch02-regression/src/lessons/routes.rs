use crate::lessons::index::get_index;
use crate::lessons::l2_prepare_data::get_lesson_2;
use crate::lessons::l3_linear_regression::get_lesson_3;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

/// Creates the lesson routes of the web application.
///
/// # Arguments
///
/// * `config`: Configuration of the web service.
pub fn lesson_routes() -> Router {
  let app = Router::new();

  app
    .route("/", get(get_index))
    .route("/lesson-2", get(get_lesson_2))
    .route("/lesson-3", get(get_lesson_3))
    .nest_service("/public", ServeDir::new("public"))
}
