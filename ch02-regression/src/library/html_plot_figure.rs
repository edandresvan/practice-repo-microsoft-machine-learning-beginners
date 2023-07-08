use maud::{html, Markup, PreEscaped};
use plotly::{Plot, Trace};

use crate::application_error::GenericResult;

pub fn single_html_figure(
  traces: Vec<Box<dyn Trace>>,
  caption: &str,
) -> GenericResult<Markup> {
  let mut plot = Plot::new();
  plot.add_traces(traces);

  Ok(html! {
    figure {
      image { ( PreEscaped(plot.to_inline_html(None)) ) }
      caption { (format!("{}", caption)) }
    }

  })
}
