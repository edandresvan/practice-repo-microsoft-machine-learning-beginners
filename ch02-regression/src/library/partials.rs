use crate::application_error::GenericResult;
use maud::{html, Markup, PreEscaped, DOCTYPE};

/// Create an HTML page using the Maud library.
///
/// # Arguments
///
/// * `page_title`: Title of the page
/// * `article_elements`: Collection of HTML elements to build an article element for the notebook.
///
/// # Returns
///
/// A result containing the HTML page.
pub fn create_html_notebook(
  page_title: &str,
  article_elements: Vec<PreEscaped<String>>,
) -> GenericResult<Markup> {
  // Build the article with the given elements
  let article = html!({
    article {
      @for element in &article_elements {
        (element)
      }
    }
  });

  let page = html!({

    (DOCTYPE)
    head {
      meta charset="utf-8"
      meta { title { (page_title) } }
      link rel="stylesheet" href="/public/styles.css";
      (PreEscaped("<script src=\"https://cdn.plot.ly/plotly-2.24.1.min.js\" charset=\"utf-8\"></script>"))
    }
    body {
      (article)
    }
  });

  Ok(page)
}

/// Create an HTML page using the Maud library.
///
/// # Arguments
///
/// * `page_title`: Title of the page
/// * `page_elements`: List of HTML elements of the page.
///
/// # Returns
///
/// A result containing the HTML page.
pub fn create_html_page(
  page_title: &str,
  page_elements: Vec<PreEscaped<String>>,
) -> GenericResult<Markup> {
  let page = html!({
    (DOCTYPE)
    head {
      meta charset="utf-8"
      meta { title { (page_title) } }
      link rel="stylesheet" href="/public/styles.css";
      (PreEscaped("<script src=\"https://cdn.plot.ly/plotly-2.24.1.min.js\" charset=\"utf-8\"></script>"))
    }
    body {
      @for element in &page_elements {
        (element)
      }
    }
  });

  Ok(page)
}
