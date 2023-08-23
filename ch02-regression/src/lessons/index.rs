use axum::{http::StatusCode, response::IntoResponse};
use linear_regression::{application_error::GenericResult, partials::create_html_page};
use maud::{html, PreEscaped};

pub async fn get_index() -> GenericResult<impl IntoResponse> {
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
    (
      StatusCode::OK,
      create_html_page("Index of Notebooks", page_elements)?,
    )
      .into_response(),
  )
}
