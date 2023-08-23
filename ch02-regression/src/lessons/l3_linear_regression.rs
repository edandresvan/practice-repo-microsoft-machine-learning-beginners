use axum::http::StatusCode;
use axum::response::IntoResponse;
use linear_regression::application_error::GenericResult;
use linear_regression::html_dataframe::html_dataframe;
use linear_regression::html_plot_figure::html_plot_figure;
use linear_regression::partials::create_html_notebook;
use linear_regression::regression_functions::RegressionModel;
use linear_regression::sample_options::SampleOptions;
use maud::{html, PreEscaped};
use ndarray::Array1;
use plotly::color::NamedColor;
use plotly::common::{Marker, Title};
use plotly::layout::Axis;
use plotly::Scatter;
use plotly::Trace;
use plotly::{Bar, Layout};
use polars::export::chrono::*;
use polars::prelude::*;
use polars::prelude::{CsvReader, DataFrame, SerReader};
use std::collections::HashMap;

/// Gets the notebook for the lesson 3 Linear Regression
///
pub async fn get_lesson_3() -> GenericResult<impl IntoResponse> {
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
    use_earliest: Some(true),
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
    ( html_dataframe(&pumpkins, Some(
    SampleOptions::builder().sample_size(10).shuffle(true).build() ) )? )
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
              // If the original value is unavailable (NULL or None) there is no need
              // to calculate the number of days, so return None
              let current_date = match date_op {
                Some(value) => value,
                None => return None,
              };

              // There is a date value, so try to calculate the number of days.
              // Otherwise, if an error ocurrs return None
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
                                                  h3 { "Get average price, month,
                                                    and day of year" }
                                                    ( html_dataframe(&pumpkins,
                                                                     Some(
  SampleOptions::builder().sample_size(15).shuffle(true).build() ) )? )
      }));

  // Adjust the price based on  the bushel size
  pumpkins = pumpkins
    .lazy()
    .rename(["Price"], ["OldPrice"])
    .select([
      all(),
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
    ( html_plot_figure(traces, &layout, "Scatter plot price vs month.")? )
  });

  // Plot price and day of the year
  let days_of_year: Vec<Option<i64>> = pumpkins["DayOfYear"].i64()?.into_iter().collect();
  let trace = Scatter::new(days_of_year, prices).mode(plotly::common::Mode::Markers);
  let traces: Vec<Box<dyn Trace>> = vec![trace];

  let layout = Layout::new()
    .title(Title::new("Price vs Day of Year"))
    .x_axis(Axis::new().title(Title::new("Day of Year")))
    .y_axis(Axis::new().title(Title::new("Price")));

  article_elements.push(html!( {
    h3 { "Plot price and day of year" }
    p { "Available data is from August through December" }
    ( html_plot_figure(traces, &layout, "Scatter plot price vs day of year.")? )
  }));

  // Calculate the correlation
  let correlation_source_data = pumpkins
    .clone()
    .lazy()
    .select([
      col("Price"),
      col("Month").cast(DataType::Float64),
      col("DayOfYear").cast(DataType::Float64),
    ])
    .collect()?;

  let correlation_month_price = correlation_source_data
    .lazy()
    .select([
      pearson_corr(col("Month"), col("Price"), 1).alias("Correlation Month vs Price"),
      pearson_corr(col("DayOfYear"), col("Price"), 1)
        .alias("Correlation DayOfYear vs Price"),
    ])
    .collect()?;

  article_elements.push(html!( {
    h2 { "Correlation" }
    ( html_dataframe(&correlation_month_price, None )? )
  }));

  // Plot a scatter plot of day of year vs price and variety
  let varieties = HashMap::<&str, NamedColor>::from([
    ("PIE TYPE", NamedColor::Red),
    ("MINIATURE", NamedColor::Blue),
    ("FAIRYTALE", NamedColor::Green),
    ("MIXED HEIRLOOM VARIETIES", NamedColor::Yellow),
  ]);

  let mut traces: Vec<Box<dyn Trace>> = Vec::new();

  for (variety_name, variety_color) in varieties.into_iter() {
    let variety_data = &pumpkins
      .clone()
      .lazy()
      .filter(col("Variety").eq(lit(variety_name)))
      .collect()?;

    let x_values = variety_data.clone()["DayOfYear"]
      .i64()?
      .into_iter()
      .collect();
    let y_values = variety_data.clone()["Price"].f64()?.into_iter().collect();

    let trace = Scatter::new(x_values, y_values)
      .mode(plotly::common::Mode::Markers)
      .name(variety_name)
      .marker(Marker::new().color(variety_color));

    traces.push(trace);
  }

  let layout = Layout::new()
    .title(Title::new("Price vs Day of Year per Variety"))
    .x_axis(Axis::new().title(Title::new("Day of Year")))
    .y_axis(Axis::new().title(Title::new("Price")));

  article_elements.push(html!( {
    h3 { "Price per Variety" }
    ( html_plot_figure(traces, &layout, "Scatter plot price vs Day of Year per Variety.")? ) 
  }));

  // Plot the mean according to the variety
  let mean_prices = pumpkins
    .clone()
    .lazy()
    .groupby([col("Variety")])
    .agg([col("Price").mean().alias("Mean Price")])
    .collect()?;

  let x_values: Vec<Option<String>> = mean_prices["Variety"]
    .clone()
    .iter()
    .map(|s| Some(s.to_string()))
    .collect();

  let y_values: Vec<Option<f64>> = mean_prices["Mean Price"].f64()?.into_iter().collect();

  let trace = Bar::new(x_values, y_values);
  let traces: Vec<Box<dyn Trace>> = vec![trace];

  let layout = Layout::new()
    .title(Title::new("Mean Price vs Variety"))
    .x_axis(Axis::new().title(Title::new("Variety")))
    .y_axis(Axis::new().title(Title::new("Mean Price")));

  article_elements.push(html!( {
    h3 { "Mean Price per Variety" }
    ( html_plot_figure(traces, &layout, "Bar plot mean price vs Variety.")? )
  }));

  // Correlation by Variety
  let correlation_price_day = pumpkins
    .clone()
    .lazy()
    .groupby([col("Variety")])
    .agg([
      pearson_corr(col("DayOfYear").cast(DataType::Float64), col("Price"), 1)
        .alias("Correlation DayOfYear vs Variety"),
    ])
    .collect()?;

  article_elements.push(html!( {
    h3 { "Correlation Price - Day of Year per Variety" }
    ( html_dataframe(&correlation_price_day, None )? )
  }));

  // Prepare data for Linear Regresion
  let pie_pumpkins = pumpkins
    .lazy()
    .filter(col("Variety").eq(lit("PIE TYPE")))
    .select([
      col("Package"),
      col("Variety"),
      col("City Name"),
      col("Low Price"),
      col("High Price"),
      col("Date"),
      col("Month").cast(DataType::Float64),
      col("DayOfYear").cast(DataType::Float64),
      col("Price"),
    ])
    .collect()?;

  article_elements.push(html! {
    h2 { "Linear Regression" }
    h3 { "Data for the Linear Regression" }
    ( html_dataframe(&pie_pumpkins, Some( SampleOptions::builder().sample_size(12).shuffle(true).build() ) )? )
  });

  // Linear Regression
  // The shape will be [n, 1]
  let x_values = pie_pumpkins
    .clone()
    .lazy()
    .select([col("DayOfYear")])
    .collect()?
    .to_ndarray::<Float64Type>(IndexOrder::Fortran)?;

  // The shape will be [n, 1]
  let y_values = pie_pumpkins
    .lazy()
    .select([col("Price")])
    .collect()?
    .to_ndarray::<Float64Type>(IndexOrder::Fortran)?;

  let col_parameters = "Parameters (β)";
  let col_r2 = "Coef Determination\n(r²)";
  let col_mse = "Mean Squared Error\n(MSE)";
  // let col_rss = Series::new_empty("RSS", &DataType::Float64);
  let col_mean_error = "Mean Error";
  let col_library = "Regression Library";

  let mut regression_results_df = DataFrame::new(vec![
    Series::new_empty(col_library, &DataType::Utf8),
    Series::new_empty(col_parameters, &DataType::Utf8),
    Series::new_empty(col_r2, &DataType::Float64),
    Series::new_empty(col_mse, &DataType::Float64),
    Series::new_empty(col_mean_error, &DataType::Utf8),
  ])?;

  // Linear Regression Using Linfa
  {
    use linfa::prelude::*;
    use linfa_linear::LinearRegression;
    use ndarray::s;

    // This is trick of using slice() instead of column() because, currently, linfa linear regression requires a Array2 [usize, 2].
    let x_values = x_values.slice(s![.., 0..1]).to_owned();

    let y_values = y_values.column(0).to_owned();

    let dataset = Dataset::new(x_values, y_values);

    // Split dataset into training/test (80%/20%)
    let (dataset_train, dataset_test) = dataset.split_with_ratio(0.8);

    let model = LinearRegression::new().fit(&dataset_train)?;

    // copied() replaces map(|v| *v)
    let correlation_coefficients: Vec<f64> = model.params().iter().copied().collect();
    let line_intercept = model.intercept();

    let predictions: Array1<f64> = model.predict(&dataset_test);

    let mse = predictions.mean_squared_error(&dataset_test.targets())?;

    let mean_error = f64::sqrt(predictions.mean_squared_error(&dataset_test.targets())?);

    let score = predictions.r2(&dataset_test.targets())?;

    let linfa_results_df = DataFrame::new(vec![
      Series::new(col_library, &["Linfa"]),
      Series::new(
        col_parameters,
        &[RegressionModel::β_to_string(
          vec![vec![line_intercept], correlation_coefficients].concat(),
        )],
      ),
      Series::new(col_r2, &[score]),
      Series::new(col_mse, &[mse]),
      Series::new(
        col_mean_error,
        &[format!(
          "{:.3} ({:.3} %)",
          mean_error,
          mean_error / predictions.mean().unwrap_or(0.0) * 100.0
        )],
      ),
    ])?;

    regression_results_df.vstack_mut(&linfa_results_df)?;

    // Draw the linear regression model
    article_elements.push(html!( {
      h3 { "Linear Regression with Linfa" }
    }));

    let days_of_year: Vec<f64> = dataset_test
      .records()
      .column(0)
      .into_iter()
      .copied()
      .collect();
    let prices = dataset_test.targets().into_iter().copied().collect();
    let predicted_prices = predictions.into_raw_vec();

    let scatter_trace =
      Scatter::new(days_of_year.clone(), prices).mode(plotly::common::Mode::Markers);
    let line_trace = Scatter::new(days_of_year.clone(), predicted_prices)
      .mode(plotly::common::Mode::Lines)
      .name("Lines");

    let traces: Vec<Box<dyn Trace>> = vec![scatter_trace, line_trace];
    let layout = Layout::new()
      .title(Title::new("Price vs Day of Year"))
      .x_axis(Axis::new().title(Title::new("Day of Year")))
      .y_axis(Axis::new().title(Title::new("Price")));
    article_elements.push(html! {
    (html_plot_figure(traces, &layout, "Scatter plot price vs day of year.")?)
    });
  }
  // Linear Regression using Smartcore
  {
    use smartcore::linear::linear_regression::*;
    use smartcore::metrics::mean_squared_error;
    use smartcore::metrics::r2::R2;
    use smartcore::metrics::Metrics;
    use smartcore::model_selection::train_test_split;

    // Split dataset into training/test (80%/20%)
    let (x_train, x_test, y_train, y_test) =
      train_test_split(&x_values, &y_values.column(0).to_vec(), 0.2, true, Some(1));

    let model =
      LinearRegression::fit(&x_train, &y_train, LinearRegressionParameters::default())?;

    // copied() replaces map(|v| *v)
    let correlation_coefficients: Vec<f64> =
      model.coefficients().iter().copied().collect();
    let line_intercept = *model.intercept();

    let predictions = model.predict(&x_test)?;

    let mse = mean_squared_error(&y_test, &predictions);

    let mean_error = f64::sqrt(mean_squared_error(&y_test, &predictions));

    let score = R2::new().get_score(&y_test.to_vec(), &predictions.to_vec());

    let smartcore_results_df = DataFrame::new(vec![
      Series::new(col_library, &["SmartCore"]),
      Series::new(
        col_parameters,
        &[RegressionModel::β_to_string(
          vec![vec![line_intercept], correlation_coefficients].concat(),
        )],
      ),
      Series::new(col_r2, &[score]),
      Series::new(col_mse, &[mse]),
      Series::new(
        col_mean_error,
        &[format!(
          "{:.3} ({:.3} %)",
          mean_error,
          mean_error / Array1::from_vec(predictions.clone()).mean().unwrap_or(0.0)
            * 100.0
        )],
      ),
    ])?;

    regression_results_df.vstack_mut(&smartcore_results_df)?;

    // Draw the linear regression model
    article_elements.push(html!( {
      h3 { "Linear Regression with SmartCore" }
    }));

    let days_of_year: Vec<f64> = x_test.column(0).into_iter().copied().collect();
    let prices = y_test;
    let predicted_prices = predictions;

    let scatter_trace =
      Scatter::new(days_of_year.clone(), prices).mode(plotly::common::Mode::Markers);
    let line_trace = Scatter::new(days_of_year.clone(), predicted_prices)
      .mode(plotly::common::Mode::Lines)
      .name("Lines");

    let traces: Vec<Box<dyn Trace>> = vec![scatter_trace, line_trace];
    let layout = Layout::new()
      .title(Title::new("Price vs Day of Year"))
      .x_axis(Axis::new().title(Title::new("Day of Year")))
      .y_axis(Axis::new().title(Title::new("Price")));
    article_elements.push(html! {
    (html_plot_figure(traces, &layout, "Scatter plot price vs day of year.")?)
    });
  }

  // Linear Regression using Matrix Math
  let mut model = RegressionModel::new(x_values.clone(), y_values.clone(), 0.8);

  model.solve(1);
  let r2 = model.r2(&model.x_test, &model.y_test);

  let mse = model.mse(&model.x_test, &model.y_test);

  let mean_error: f64 = mse.sqrt();

  let predictions = model.predict(&model.x_test);

  let matrix_math_results_df = DataFrame::new(vec![
    Series::new(col_library, &["Matrix Math"]),
    Series::new(
      col_parameters,
      &[RegressionModel::β_to_string(model.β.column(0).to_vec())],
    ),
    Series::new(col_r2, &[r2]),
    Series::new(col_mse, &[mse]),
    Series::new(
      col_mean_error,
      &[format!(
        "{:.3} ({:.3} %)",
        mean_error,
        mean_error / predictions.column(0).mean().unwrap_or(0.0) * 100.0
      )],
    ),
  ])?;

  regression_results_df.vstack_mut(&matrix_math_results_df)?;

  // Draw the linear regression model
  article_elements.push(html!( {
    h3 { "Linear Regression with Matrix Math" }
  }));

  // The days of year are in the column 1, because it is an expansion matrix X = [1, x].
  let days_of_year: Vec<f64> = model.x_test.column(1).into_iter().copied().collect();
  let prices: Vec<f64> = model.y_test.column(0).into_iter().copied().collect();
  let predicted_prices: Vec<f64> = predictions.column(0).into_iter().copied().collect();

  let scatter_trace =
    Scatter::new(days_of_year.clone(), prices).mode(plotly::common::Mode::Markers);
  let line_trace = Scatter::new(days_of_year.clone(), predicted_prices)
    .mode(plotly::common::Mode::Lines)
    .name("Lines");

  let traces: Vec<Box<dyn Trace>> = vec![scatter_trace, line_trace];
  let layout = Layout::new()
    .title(Title::new("Price vs Day of Year"))
    .x_axis(Axis::new().title(Title::new("Day of Year")))
    .y_axis(Axis::new().title(Title::new("Price")));
  article_elements.push(html! {
  (html_plot_figure(traces, &layout, "Scatter plot price vs day of year.")?)
  });

  article_elements.push(html! {
    h3 { "Linear Regression Results" }
    ( html_dataframe(&regression_results_df, None)?  )
  });

  Ok(
    (
      StatusCode::OK,
      create_html_notebook("Lesson 3: Linear Regression", article_elements)?,
    )
      .into_response(),
  )
}
