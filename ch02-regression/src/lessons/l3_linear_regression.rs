use actix_web::HttpResponse;
use linear_regression::application_error::GenericResult;
use linear_regression::html_dataframe::html_dataframe;
use linear_regression::partials::create_html_notebook;
use linear_regression::sample_options::SampleOptions;
use maud::{html, PreEscaped};
use polars::export::chrono::*;
use polars::prelude::*;
use polars::prelude::{CsvReader, DataFrame, SerReader};
use linear_regression::html_plot_figure::html_plot_figure;
use plotly::common::Title;
use plotly::Layout;
use plotly::Trace;
use plotly::layout::Axis;
use plotly::Scatter;

/// Gets the notebook for the lesson 3 Linear Regression
///
pub async fn get_lesson_3() -> GenericResult<HttpResponse> {
  // List containing the sections and elements of a HTML article fof data analysis.
  let mut article_elements: Vec<PreEscaped<String>> = Vec::new();

  // Load the dataset
  let mut pumpkins: DataFrame = CsvReader::from_path("data/US-pumpkins.csv")?
    .has_header(true)
    .infer_schema(Some(2000))
    .finish()?;

  // Filter only the packages by bushel.
  pumpkins = pumpkins
    .lazy()
    .filter(col("Package").str().contains(lit("bushel"), true))
    .collect()?;

   // Select the desired attributes
   pumpkins = pumpkins
   .lazy()
   .select([
     col("Package"),
     col("Variety"),
     col("City Name"),
     col("Low Price"),
     col("High Price"),
     col("Date"),
   ])
   .collect()?;

  // Options for converting a string into a datetime
  let dt_options = StrptimeOptions {
    format: Some("%m/%d/%y".to_string()),
    strict: true,
    exact: true,
    cache: true,
  };

  // Transform into proper date datatype.
  pumpkins = pumpkins
    .lazy()
    .with_columns([col("Date").str().strptime(DataType::Date, dt_options)])
    .collect()?;

  article_elements.push(html! {
    h1 { "Lesson 3: Linear and Polynomial Regression for Pumpkin Pricing" }
    h2 { "Prepare the Dataset" }
    h3 { "Load and convert the source data" }
    ( html_dataframe(&pumpkins, Some( SampleOptions::builder().sample_size(10).shuffle(true).build() ) )? ) 
  });

   // Calculate average price
   pumpkins = pumpkins
   .lazy()
   .with_columns([((col("Low Price") + col("High Price")) / lit(2.0)).alias("Price")])
   .collect()?;

  // Extract the month
  let months = pumpkins
    .clone()
    .lazy()
    .select([col("Date").dt().month().alias("Month")])
    .collect()?;

  // See: https://stackoverflow.com/questions/76059689/convert-str-to-f64-using-a-rust-polars-custom-function
  let days_of_year = pumpkins
    .clone()
    .lazy()
    .select([col("Date")
      .map(
        |dates_col| {
          let days_col = dates_col
            .date()?
            .as_date_iter()
            .map(|date_op| {
              // If the original value is unavailable (NULL or None) there is no need to calculate the number of days, so return None
              let current_date = match date_op {
                Some(value) => value,
                None => return None,
              };

              // There is a date value, so try to calculate the number of days. Otherwise, if an error ocurrs return None
              let year_start = match NaiveDate::from_ymd_opt(current_date.year(), 1, 1) {
                Some(value) => value,
                None => return None,
              };

              // Finally, return the number of days encapsulated in an Option enum due to we are considering returning None values as mentioned above
              Some((current_date - year_start).num_days())
            })
            .collect();

          Ok(Some(days_col))
        },
        GetOutput::from_type(DataType::Int32),
      )
      .alias("DayOfYear")])
    .collect()?;

    pumpkins = polars::functions::hor_concat_df(&[pumpkins, months, days_of_year])?;

    article_elements.push(html!( {
      h3 { "Get average price, month, and day of year" }
      ( html_dataframe(&pumpkins, Some( SampleOptions::builder().sample_size(15).shuffle(true).build() ) )? ) 
    }));


    // Adjust the price based on  the bushel size
    pumpkins = pumpkins
    .lazy()
    .rename(["Price"], ["OldPrice"])
    .select([ all(),     
      when(col("Package").str().contains(lit("1 1/9"), false))
        .then(col("OldPrice") / lit(1.0 + 1.0 / 9.0))
        .when(col("Package").str().contains(lit("1/2"), false))
        .then(col("OldPrice") / lit(1.0 / 2.0))
        .otherwise(col("OldPrice"))
        .alias("Price"),
     ])
    .drop_columns(["OldPrice"])
    
    .collect()?;


  article_elements.push(html!( {
    h3 { "Adjust price based on  the bushel size" }
    ( html_dataframe(&pumpkins, Some( SampleOptions::builder().sample_size(15).shuffle(true).build() ) )? )
  }));

  // Plot price and month
  let months: Vec<Option<u32>> = pumpkins["Month"].u32()?.into_iter().collect();
  let prices: Vec<Option<f64>> = pumpkins["Price"].f64()?.into_iter().collect();

  let trace = Scatter::new(months, prices.clone()).mode(plotly::common::Mode::Markers);
  let traces: Vec<Box<dyn Trace>> = vec![trace];

  let layout = Layout::new()
    .title(Title::new("Price vs Month"))
    .x_axis(Axis::new().title(Title::new("Month")))
    .y_axis(Axis::new().title(Title::new("Price")));

  article_elements.push(html! {
    h2 { "Scatter Plots" }
    h3 { "Plot price and month" }
    p { "Available data is from August through December" }
    ( html_plot_figure(traces, layout, "Scatter plot price vs month.")? ) 
  });

  // Plot price and day of the year
  let days_of_year: Vec<Option<i64>> = pumpkins["DayOfYear"].i64()?.into_iter().collect();
  let trace =
    Scatter::new(days_of_year, prices.clone()).mode(plotly::common::Mode::Markers);
  let traces: Vec<Box<dyn Trace>> = vec![trace];

  let layout = Layout::new()
    .title(Title::new("Price vs Day of Year"))
    .x_axis(Axis::new().title(Title::new("Day of Year")))
    .y_axis(Axis::new().title(Title::new("Price")));
  

  article_elements.push(html!( {
    h3 { "Plot price and day of year" }
    p { "Available data is from August through December" }
    ( html_plot_figure(traces, layout, "Scatter plot price vs day of year.")? ) 
  }));

  Ok(HttpResponse::Ok().body(
    create_html_notebook("Lesson 3: Linear Regression", article_elements)?.into_string(),
  ))
}
