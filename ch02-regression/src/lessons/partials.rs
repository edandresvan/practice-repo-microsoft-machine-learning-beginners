use linear_regression::application_error::GenericResult;
use maud::{html, Markup, PreEscaped, DOCTYPE};

/// Create an HTML page using the Maud library.
/// 
/// # Arguments
/// 
/// * `page_title`: Title of the page
/// * `article`: HTML article object containing the elements to show in the page.
/// 
/// # Returns
/// 
/// A result containing the HTML page.
pub fn create_html_page(
  page_title: &str,
  article: Markup,
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
      (article)
    }
  });

  Ok(page)
}
