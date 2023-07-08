use actix_web::HttpResponse;
use linear_regression::application_error::ApplicationError;
use maud::html;

use super::partials::create_html_page;

pub async fn get_index() -> Result<HttpResponse, ApplicationError> {
  let article = html!({

    article {
      h1 { "Lessons" }
      ol {
        li { a href="/lesson-2" { "Lesson 2" }  }
        li { a href="/lesson-3" { "Lesson 3" }  }
      }
    }

  });

  let page = create_html_page("Index of Notebooks", article)?;

  Ok(HttpResponse::Ok().body(page.into_string()))
}
