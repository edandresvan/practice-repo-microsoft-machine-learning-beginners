//  GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Represents a generic result for the system.
pub type GenericResult<T> = Result<T, ApplicationError>;

/// Represents an error ocurred in the application.
pub enum ApplicationError {
  /// An error from the Polars library.
  PolarsError(polars::error::PolarsError),
  // An error parsing an integer value.
  ParseIntError(std::num::ParseIntError),
  // An error of the standard input-output (IO).
  IOError(std::io::Error),
  // An error when reading an environment variable.
  EnvVarError(std::env::VarError),
  // An error from the Linfa library.
  LinfaError(linfa::error::Error),
  // An error from the Linfa linear library.
  LinfaLinearError(linfa_linear::LinearError<f64>),
  // An error from the SmartCore library.
  SmartCoreError(smartcore::error::Failed),
  // Shape error from the ndarray library.
  NDArrayShapeError(ndarray::ShapeError),
  // An error from the CSV library.
  CSVError(csv::Error),
  /// Any kind of error ocurred.
  GenericError(GenericError),
}

impl std::error::Error for ApplicationError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::PolarsError(err) => Some(err),
      Self::ParseIntError(err) => Some(err),
      Self::IOError(err) => Some(err),
      Self::EnvVarError(err) => Some(err),
      Self::LinfaError(err) => Some(err),
      Self::LinfaLinearError(err) => Some(err),
      Self::NDArrayShapeError(err) => Some(err),
      Self::SmartCoreError(err) => Some(err),
      Self::CSVError(err) => Some(err),
      Self::GenericError(err) => Some(err.as_ref()),
    }
  }
}

impl std::fmt::Debug for ApplicationError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::PolarsError(err) => write!(f, "Polars Error: {:?}", err),
      Self::ParseIntError(err) => write!(f, "Parsing Integer Error: {:?}", err),
      Self::IOError(err) => write!(f, "Standard Input/Output Error: {:?}", err),
      Self::EnvVarError(err) => write!(f, "Environment Variable Error: {:?}", err),
      Self::LinfaError(err) => write!(f, "Linfa Error: {:?}", err),
      Self::LinfaLinearError(err) => write!(f, "Linfa Linear Error: {:?}", err),
      Self::NDArrayShapeError(err) => write!(f, "Ndarray Shape Error: {:?}", err),
      Self::CSVError(err) => write!(f, "CVS Library Error: {:?}", err),
      Self::SmartCoreError(err) => write!(f, "SmartCore Library Error: {:?}", err),
      Self::GenericError(err) => write!(f, "GenericError: {:?}", err),
    }
  }
}

impl std::fmt::Display for ApplicationError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::PolarsError(err) => write!(f, "Polars Error: {}", err),
      Self::ParseIntError(err) => write!(f, "Parsing Integer Error: {:?}", err),
      Self::IOError(err) => write!(f, "Standard Input/Output Error: {}", err),
      Self::EnvVarError(err) => write!(f, "Environment Variable Error: {}", err),
      Self::LinfaError(err) => write!(f, "Linfa Error: {}", err),
      Self::LinfaLinearError(err) => write!(f, "Linfa Error: {}", err),
      Self::NDArrayShapeError(err) => write!(f, "Ndarray Shape Error: {:}", err),
      Self::CSVError(err) => write!(f, "CVS Error: {}", err),
      Self::SmartCoreError(err) => write!(f, "SmartCore Library Error: {}", err),
      Self::GenericError(err) => write!(f, "GenericError: {}", err),
    }
  }
}

impl From<polars::error::PolarsError> for ApplicationError {
  fn from(value: polars::error::PolarsError) -> Self {
    Self::PolarsError(value)
  }
}

impl From<std::num::ParseIntError> for ApplicationError {
  fn from(value: std::num::ParseIntError) -> Self {
    Self::ParseIntError(value)
  }
}

impl From<std::io::Error> for ApplicationError {
  fn from(value: std::io::Error) -> Self {
    Self::IOError(value)
  }
}

impl From<csv::Error> for ApplicationError {
  fn from(value: csv::Error) -> Self {
    Self::CSVError(value)
  }
}

impl From<std::env::VarError> for ApplicationError {
  fn from(value: std::env::VarError) -> Self {
    Self::EnvVarError(value)
  }
}

impl From<linfa::error::Error> for ApplicationError {
  fn from(value: linfa::error::Error) -> Self {
    Self::LinfaError(value)
  }
}

impl From<linfa_linear::LinearError<f64>> for ApplicationError {
  fn from(value: linfa_linear::LinearError<f64>) -> Self {
    Self::LinfaLinearError(value)
  }
}

impl From<ndarray::ShapeError> for ApplicationError {
  fn from(value: ndarray::ShapeError) -> Self {
    Self::NDArrayShapeError(value)
  }
}

impl From<smartcore::error::Failed> for ApplicationError {
  fn from(value: smartcore::error::Failed) -> Self {
    Self::SmartCoreError(value)
  }
}

/// Error type for generic operations that could result in PolarsError::External
pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

impl From<GenericError> for ApplicationError {
  fn from(value: GenericError) -> Self {
    ApplicationError::GenericError(value)
  }
}

impl ApplicationError {
  /// Sends a user-friendly text message to inform that an error has ocurred.
  fn error_response(&self) -> String {
    self.to_string()
  }
}

impl actix_web::error::ResponseError for ApplicationError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    // Specify the HTTP status code for the response message.
    match self {
      ApplicationError::PolarsError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
      ApplicationError::ParseIntError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
      ApplicationError::IOError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
      ApplicationError::EnvVarError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
      ApplicationError::CSVError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
      ApplicationError::LinfaError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
      ApplicationError::LinfaLinearError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
      ApplicationError::NDArrayShapeError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
      ApplicationError::SmartCoreError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
      ApplicationError::GenericError(_) => {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
      }
    }
  }

  fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
    // Build the body for the response using the status code and the error message.
    actix_web::HttpResponse::build(self.status_code()).body(self.error_response())
  }
}
