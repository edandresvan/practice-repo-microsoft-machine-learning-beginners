use linear_regression::application_error::GenericResult;
use maud::{Markup, html, DOCTYPE, PreEscaped};


pub fn create_html_page(page_title: &str, article: Markup) -> GenericResult<Markup> {
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