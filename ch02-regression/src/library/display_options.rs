use crate::application_error::GenericResult;

#[derive(Clone)]
pub struct DisplayOptions {
  pub column_width: usize,
}

impl DisplayOptions {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn create_from_environment_variables() -> GenericResult<Self> {
    let mut options = Self::new();

    if let Ok(column_width) = std::env::var("COLUMN_WIDTH")
      .unwrap_or("".to_string())
      .parse::<usize>()
    {
      options.column_width = column_width;
    }

    Ok(options)
  }
}

impl std::default::Default for DisplayOptions {
  fn default() -> Self {
    Self { column_width: 100 }
  }
}
