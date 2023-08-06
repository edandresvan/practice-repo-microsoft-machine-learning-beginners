use maud::{html, Markup, PreEscaped};
use plotly::{Layout, Plot, Trace};

use crate::application_error::GenericResult;

/// Generates a HTML figure for the given Plotly `[plotly::plot::Plot]` object.
///
/// # Arguments
///
/// * `traces`: Set of traces to draw.
/// * `layout`: Layout of the final plot generated.
/// * `caption`: Caption text of the figure.
pub fn html_plot_figure(
  traces: Vec<Box<dyn Trace>>,
  layout: &Layout,
  caption: &str,
) -> GenericResult<Markup> {
  let mut plot = Plot::new();
  plot.set_configuration(plot.configuration().clone().responsive(true));
  plot.add_traces(traces);
  plot.set_layout(layout.clone().auto_size(true));

  Ok(html! {
    figure .plot-figure {
      div .embeded-plot { ( PreEscaped(plot.to_inline_html(None)) ) }
      figcaption .plot-figure-caption { (format!("{}", caption)) }
    }

  })
}
