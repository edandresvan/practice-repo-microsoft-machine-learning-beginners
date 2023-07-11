use actix_web::HttpResponse;
use linear_regression::{
  application_error::ApplicationError, html_dataframe::html_dataframe,
  html_plot_figure::html_plot_figure, sample_options::SampleOptions,
};
use maud::{html, PreEscaped};
use plotly::{common::{Mode, Title}, Scatter, Trace, Layout, layout::Axis, Bar};
use polars::prelude::*;

use crate::lessons::partials::create_html_page;

pub async fn get_lesson_2() -> Result<HttpResponse, ApplicationError> {
  // Builder for the article containing the data analysis sections and elements.
  let mut page_elements: Vec<PreEscaped<String>> = Vec::new();

  // Load the dataset
  let df: DataFrame = CsvReader::from_path("data/US-pumpkins.csv")?
    .has_header(true)
    .infer_schema(Some(2000))
    .finish()?;

  // Describe the dataset and explore some samples
  page_elements.push(html! {
    h2 { "1. Load the dataset" }
    h3 { "Describe the dataset"}
    div { ( html_dataframe(&df.describe(None)?, None)? ) }
    h3 { "Load a sample data from the dataset " }
    ( html_dataframe(&df, Some( SampleOptions::builder().sample_size(10).shuffle(true).build() ) )? )
  });

  // Exploration Strategies
  page_elements.push(html! {
    h2 { "2. Exploration Strategies" }
    h3 { "Verify which attributes (columns) has null values" }
    ( html_dataframe(&df.null_count(), None)? )
  });

  // Select the attributes of packages, prices and date
  let pumpkins = df.select(["Package", "Low Price", "High Price", "Date"])?;

  // Options for converting a string into a datetime
  let dt_options = StrptimeOptions {
    format: Some("%m/%d/%y".to_string()),
    strict: true,
    exact: true,
    cache: true,
    // tz_aware: false,
    // utc: false,
  };

  // Ensure the date attribute (column) is a date properly.
  let pumpkins = pumpkins
    .lazy()
    .with_column(col("Date").str().strptime(DataType::Date, dt_options))
    .with_columns([(col("Low Price").alias("Price") + col("High Price")) / lit(2)])
    .collect()?;

  page_elements.push(html! {
    h3 { "Select the attributes of packages, average price, and date" }
    ( html_dataframe(&pumpkins, Some( SampleOptions::builder().sample_size(10).shuffle(true).build() ) )? )
  });

  // Extract the month from the date and create a new dataframe.
  let pumpkins = pumpkins
    .lazy()
    .with_column(col("Date").alias("Month").dt().month())
    .collect()?
    .select(["Package", "Low Price", "High Price", "Price", "Month"])?;

  page_elements.push(html!{
      h3 { "Extract the month from the date and create a new dataframe" }
      ( html_dataframe(&pumpkins, Some( SampleOptions::builder().sample_size(10).shuffle(true).build() ) )? )
    });

  // Filter the pumpkins packaged in bushels
  let pumpkins = pumpkins
    .lazy()
    .filter(col("Package").str().contains(lit("bushel"), true))
    .collect()?;

  page_elements.push(html! {
    h3 { "Filter the pumpkins packaged in bushels" }
    ( html_dataframe(&pumpkins, Some( SampleOptions::builder().sample_size(15).shuffle(true).build() ) )? )
  });

  // Adjust the price according to the size of the bushel
  let pumpkins = pumpkins
    .lazy()
    .rename(["Price"], ["OldPrice"])
    .with_column(
      when(col("Package").str().contains(lit("1 1/9"), false))
        .then(col("OldPrice") / lit(1.0 + 1.0 / 9.0))
        .when(col("Package").str().contains(lit("1/2"), false))
        .then(col("OldPrice") / lit(1.0 / 2.0))
        .otherwise(col("OldPrice"))
        .alias("Price"),
    )
    .drop_columns(["OldPrice"])
    .select([
      col("Package"),
      col("Low Price"),
      col("High Price"),
      col("Price"),
      col("Month"),
    ])
    .collect()?;

  page_elements.push(html! {
    h3 { "Adjust the price according to the size of the bushel" }
    ( html_dataframe(&pumpkins, Some( SampleOptions::builder().sample_size(15).shuffle(true).build() ) )? ) 
  });

  // Add a Scatter Plot
  let prices = pumpkins["Price"].f64()?.into_iter().collect();
  let months = pumpkins["Month"].u32()?.into_iter().collect();

  let trace = Scatter::new(prices, months).mode(Mode::Markers);
  let traces: Vec<Box<dyn Trace>> = vec![trace];

  let layout = Layout::new()
    .title(Title::new("Price vs Month"))
    .x_axis(Axis::new().title(Title::new("Month")))
    .y_axis(Axis::new().title(Title::new("Price")));

  page_elements.push(html! {
    ( html_plot_figure(traces, layout, "Scatter plot for the pumpkins.")? ) 
  });

  // Add a Bar Plot
  let pumpkins = pumpkins.lazy().groupby(["Month"]).agg([col("Price").median()]).sort("Month", SortOptions { descending: false, nulls_last: true, multithreaded: true }).collect()?;

  let prices = pumpkins["Price"].f64()?.into_iter().collect();
  let months = pumpkins["Month"].u32()?.into_iter().collect();

  let trace = Bar::new(months, prices);
  let traces: Vec<Box<dyn Trace>> = vec![trace];

  let layout = Layout::new()
    .title(Title::new("Mean Price vs Month"))
    .x_axis(Axis::new().title(Title::new("Month")))
    .y_axis(Axis::new().title(Title::new("Price")));

  page_elements.push(html! {
    ( html_plot_figure(traces, layout, "Bar plot for the pumpkins.")? ) 
  });


  let article = html!({
    article {
      @for element in &page_elements {
        (element)
      }
    }
  });

  let page = create_html_page("Lesson 2: Preparing Source Data", article)?;

  Ok(HttpResponse::Ok().body(page.into_string()))
}
