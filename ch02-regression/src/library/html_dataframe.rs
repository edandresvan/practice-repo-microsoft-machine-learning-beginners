use std::borrow::Cow;

use maud::{html, Markup, PreEscaped};
use polars::{
  prelude::{AnyValue, DataFrame},
  series::Series,
};

use crate::{
  application_error::GenericResult,
  sample_options::SampleOptions,
};

/// Displays the given dataframe object as a HTML table.
///
/// # Arguments
///
/// * `df` - Dataframe that will be displayed.
/// * `options` - Options for displaying the dataframe.
///
/// # Returns
///
/// A `[GenericResult]` containing a `[maud::Markup]` representation of the dataframe.
pub fn html_dataframe(
  df: &DataFrame,
  options: Option<SampleOptions>,
) -> GenericResult<Markup> {
  // If no options were provided, then create default options.
  let mut df_options = if let Some(options) = options.clone() {
    options
  } else {
    SampleOptions::default()
  };

  // The requested sample cannot be greater than the total number of rows.
  if df_options.sample_size > df.height() {
    df_options.sample_size = df.height()
  }

  // Obtain a set of rows depending on the sample size and suffle options provided to this function
  let df_sample = match options.clone() {
    // If no options were provided, the user wants to display the entire dataframe.
    None => df.clone(),
    // If options were provided, then the user wants to display a subset of the dataframe rows.
    Some(_) => {
      if df_options.shuffle == false {
        // The user wants a sample of rows in sequential order, so get an slice from the dataset
        df.slice(0, df_options.sample_size)
      } else {
        // The user wants a sample of rows in random order, so get an random slice from the dataset
        df.sample_n(df_options.sample_size, false, true, None)?
      }
    }
  };

  Ok(html!({
    div .tblcon {
    table .dataframe-table {
      // Show a caption with metadata about the dataframe
      caption { ( format!("Dataframe info: rows: {0}, columns: {1}. Showing: {2} rows. Suffle: {3}", df.height(), df.width(), df_options.sample_size , (if df_options.shuffle == true { "Yes"} else { "No "}) ) ) }

      // Display the table headers containing two rows:
      // - Field name
      // - Field datatype
      thead {
        // - Display header row for the field name
        tr .field-name {
          @for field in df.fields() {
            th { ( format!("{}", field.name()) ) }
          }
        }
        // - Display header row for the field datatype
        tr .field-datatype {
          @for field in df.fields() {
            th { ( format!("{}", field.data_type()) ) }
          }
        }
      }

      // Display the data rows depeding on the options
      tbody { ( create_tbody(&df_sample, &df_options)? ) }
    }
  }
  }))
}

/// Creates the tbody rows from the given batches.
///
/// # Arguments
///
/// * `batches`: Collection of batches from which the rows and cells of the tbody element will be created.
/// * `formatting_options` Formatting options for the cells values.
///
/// # Returns
///
/// A HTLM tbody element with rows and data from the given batches.
pub fn create_tbody(
  df: &DataFrame,
  options: &SampleOptions,
) -> GenericResult<Markup> {
  let columns = df.get_columns();

  Ok(html! {

    @for row_index in 0..options.sample_size {
      tr {
        // Similar to a matrix, the pair of indexes (i, j) represents the position of a cell. Here i is the row index, and j is the column
        @for column in columns {
          @if column.dtype().is_numeric() {
            td .numeric-value { ( PreEscaped(format_series_value(column, row_index)? )) }
          } @else if column.dtype().is_temporal() {
            td .datetime-value { ( PreEscaped(format_series_value(column, row_index)? )) }
          }
          @else {
            td { ( PreEscaped(format_series_value(column, row_index)? )) }
          }
        }
      }
    }
  })
}

// used for formatting
fn format_series_value(
  serie: &Series,
  index: usize,
) -> GenericResult<Cow<str>> {
  let out = match serie.0.get(index)? {
    AnyValue::Utf8(s) => {
      let value = Cow::Borrowed(s);
      if value.parse::<f32>().is_ok() || value.parse::<f64>().is_ok() {
        Cow::Owned(format!("{:.3}", value))
      } else {
        value
      }
    },
    AnyValue::Float32(value) => Cow::Owned(format!("{:.3}", value)),
    AnyValue::Float64(value) => Cow::Owned(format!("{:.3}", value)),
    AnyValue::Decimal(value, _size) => Cow::Owned(format!("{:.3}", value)),
    AnyValue::Null => {
      Cow::Borrowed("<span class=\"null-value\">null</span>")
    },
    #[cfg(feature = "dtype-categorical")]
    AnyValue::Categorical(idx, rev, arr) => {
      if arr.is_null() {
        Cow::Borrowed(rev.get(idx))
      } else {
        unsafe { Cow::Borrowed(arr.deref_unchecked().value(idx as usize)) }
      }
    }
    av => Cow::Owned(format!("{av}")),
  };
  Ok(out)
}
