use linear_regression::partials::create_html_page;
use actix_web::HttpResponse;
use linear_regression::application_error::ApplicationError;
use maud::{html, PreEscaped};

pub async fn get_index() -> Result<HttpResponse, ApplicationError> {
  let mut page_elements: Vec<PreEscaped<String>> = Vec::new();

  page_elements.push(html!({

    article {
      h1 { "Lessons" }
      ol {
        li { a href="/lesson-2" { "Lesson 2" }  }
        li { a href="/lesson-3" { "Lesson 3" }  }
      }
    }

  }));

  Ok(
    HttpResponse::Ok()
      .body(create_html_page("Index of Notebooks", page_elements)?.into_string()),
  )
}
