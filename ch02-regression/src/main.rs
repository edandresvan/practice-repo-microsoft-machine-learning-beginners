#![allow(non_snake_case)]
use actix_web::{App, HttpServer};
use mimalloc::MiMalloc;



#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[path = "lessons/mod.rs"]
mod lessons;

use lessons::routes::lesson_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let app = move || {
    App::new()
      .service(
        actix_files::Files::new("/public", "./public")
          .use_etag(true)
          .use_last_modified(true),
      )
      .configure(lesson_routes)
  };

  // Start the server
  let server_socket = "127.0.0.1:3030";
  HttpServer::new(app).bind(server_socket)?.run().await
}
