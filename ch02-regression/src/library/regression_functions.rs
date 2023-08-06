#![allow(non_snake_case)]

use linfa_linalg::norm::Norm;
use linfa_linalg::qr::QRInto;
use ndarray::s;
use ndarray::Array;
use ndarray::Array2;
use ndarray::Ix2;
use polars::export::arrow::temporal_conversions::date32_to_date;
use polars::prelude::ListPrimitiveChunkedBuilder;

use crate::application_error::GenericResult;

/// Represents a model for a regression.
pub struct RegressionModel {
  /// Matrix of explanatory (input) variables.
  pub x: Array<f64, Ix2>,
  /// Vector of response (output) variables.
  pub y: Array2<f64>,
  /// Matrix of explanatory (input) variables built from the `x` vector.
  pub X: Array2<f64>,
  /// Vector of response (output) variables: Y = Xβ + ε.
  pub Y: Array2<f64>,
  /// Vector of unknown parameters.
  pub β: Array2<f64>,

  pub x_train: Array<f64, Ix2>,
  pub x_test: Array<f64, Ix2>,
  pub y_train: Array<f64, Ix2>,
  pub y_test: Array<f64, Ix2>,

  pub split_ratio: f32,

  pub δ2: f64,
}

impl RegressionModel {
  /// Creates a new regression instance with the given explanatory and response data matrices.
  ///
  /// # Arguments
  ///
  /// * `x`: Matrix of explanatory (input) variables.
  /// * `y`: Vector of response (output) variables.
  pub fn new(
    x: Array2<f64>,
    y: Array2<f64>,
    split_ratio: f32,
  ) -> Self {
    // Get the number of rows of the explanatory variables in order to initialize the dimensions
    // of the other vectors and matrices.
    let n_rows: usize = x.nrows();
    Self {
      x,
      y,
      X: Array2::<f64>::zeros((1, 1)),
      Y: Array2::<f64>::zeros((1, 1)),
      β: Array2::<f64>::zeros((n_rows, 1)),
      x_train: Array2::<f64>::zeros((n_rows, 1)),
      x_test: Array2::<f64>::zeros((n_rows, 1)),
      y_train: Array2::<f64>::zeros((n_rows, 1)),
      y_test: Array2::<f64>::zeros((n_rows, 1)),
      split_ratio,
      δ2: 0.0_f64,
    }
  }

  /// Solves the linear model equation Y = Xβ + ε.
  /// It calculates β: the coeficients or parameters.
  pub fn solve(
    &mut self,
    degree: i32,
  ) {
    (self.x_train, self.x_test) =
      Self::split_polyfit_data(&self.x, degree, self.split_ratio);

    (self.y_train, self.y_test) = Self::split_data(&self.y, self.split_ratio);

    self.β = self
      .x_train
      .clone()
      .reversed_axes()
      .dot(&self.x_train)
      .qr_into()
      .expect("QRError")
      .inverse()
      .expect("inverse error")
      .dot(&self.x_train.clone().reversed_axes())
      .dot(&self.y_train.clone());
    self.δ2 = (&self.y_train - &self.x_train.dot(&self.β))
      .norm_max()
      .powi(2)
      / (self.x_train.nrows() as f64);
  }

  /// Gets the vector of residuals: e = y - Xβ.
  ///
  /// # Arguments
  ///
  /// * `x`: Matrix of explanatory (input) variables.
  /// * `y`: Real, measured or observed response (output) variables. On the other hand, Xβ is
  /// the estimated or predicted `y` values by the regression.
  pub fn e(
    &self,
    x: &Array<f64, Ix2>,
    y: &Array<f64, Ix2>,
  ) -> Array<f64, Ix2> {
    y - &x.dot(&self.β)
  }

  /// Calculates the residual sum of squares (RSS). RSS = ‖ e ‖².
  pub fn rss(
    &self,
    x: &Array<f64, Ix2>,
    y: &Array<f64, Ix2>,
  ) -> f64 {
    // The euclidian norm is also known as the L2 norm
    self.e(x, y).norm_l2().powi(2)
  }

  pub fn rse(
    &self,
    x: &Array<f64, Ix2>,
    y: &Array<f64, Ix2>,
  ) -> f64 {
    let n = x.nrows();
    let d = self.β.nrows();
    let p = d + 1;

    self.rss(x, y) / ((n - p) as f64)
  }

  pub fn δ2(
    &self,
    x: &Array<f64, Ix2>,
    y: &Array<f64, Ix2>,
  ) -> f64 {
    // The euclidian norm is also known as the L2 norm
    (y - x.dot(&self.β)).norm_l2().powi(2) / (x.nrows() as f64)
  }

  pub fn r2(
    &self,
    x: &Array<f64, Ix2>,
    y: &Array<f64, Ix2>,
  ) -> f64 {
    let y_mean = y.mean().unwrap();

    // let sum_square_residuals: f64 = self.e(x, y).column(0).iter().fold(0.0_f64, |sum, e_value| sum + e_value.powi(2));
    let sum_square_residuals: f64 = self.rss(x, y);

    let sum_square_mean: f64 = y
      .column(0)
      .iter()
      .fold(0.0_f64, |sum, y_value| sum + (y_value - y_mean).powi(2));

    1.0_f64 - (sum_square_residuals / sum_square_mean)
  }

  pub fn mse(
    &self,
    x: &Array<f64, Ix2>,
    y: &Array<f64, Ix2>,
  ) -> f64 {
    let e = self.e(x, y);
    let m = e.clone().reversed_axes().dot(&e);
    (m.get((0, 0)).unwrap() / (x.nrows() as f64))
  }

  /// Predicts a vector of response (output) variables given a matrix of explanatory (input) variables.
  ///
  /// # Arguments
  ///
  /// * `x`: Matrix of explanatory (input) variables.
  ///
  /// # Returns
  ///
  /// * `y`:  Vector of response (output) variables.
  pub fn predict(
    &self,
    x: &Array<f64, Ix2>,
  ) -> Array<f64, Ix2> {
    x.dot(&self.β)
  }

  pub fn split_data(
    source_data: &Array<f64, Ix2>,
    ratio: f32,
  ) -> (Array<f64, Ix2>, Array<f64, Ix2>) {
    // Find the index to split the data
    let split_index: usize = (source_data.nrows() as f32 * ratio).ceil() as usize;
    // Get the train data from index 0 up to the split index exclusive
    let train_data = source_data.slice(s![0..split_index, ..]).to_owned();
    // Get the test data from the split index to the end
    let test_data = source_data
      .slice(s![split_index..source_data.nrows(), ..])
      .to_owned();

    (train_data, test_data)
  }

  pub fn polyfit_data(
    source_data: &Array<f64, Ix2>,
    degree: i32,
  ) -> Array<f64, Ix2> {
    // Create a matrix to hold the values of x as a polynomial
    let mut x_model = Array2::<f64>::zeros((source_data.nrows(), (degree + 1) as usize));

    // Create the columns for the powers of x
    for column_index in 0..=degree {
      ndarray::Zip::from(&mut x_model.column_mut(column_index as usize))
        .and(source_data.clone().column(0))
        .for_each(|x, a| *x = a.powi(column_index));
    }

    x_model
  }

  /// Splits the given datasource into training and testing sets given the ratio value.
  ///
  /// # Arguments
  ///
  /// * `data_source`: Data source to be splitted.
  /// * `degree`: Degree of the polynomial vector of explanatory `x` (input) variables.
  /// * `ratio`: Value of the ratio to create the train and tests sets.
  ///   For example, a ratio of 0.8 means that 80% of the source data will be for the training set
  ///   and that 20% will be for the testing set.
  pub fn split_polyfit_data(
    source_data: &Array<f64, Ix2>,
    degree: i32,
    ratio: f32,
  ) -> (Array<f64, Ix2>, Array<f64, Ix2>) {
    let x_model = Self::polyfit_data(source_data, degree);

    // Find the index to split the data
    let split_index: usize = (source_data.nrows() as f32 * ratio).ceil() as usize;
    // Get the train data from index 0 up to the split index exclusive
    let train_data = x_model.slice(s![0..split_index, ..]).to_owned();
    // Get the test data from the split index to the end
    let test_data = x_model
      .slice(s![split_index..source_data.nrows(), ..])
      .to_owned();

    (train_data, test_data)
  }

  pub fn β_to_string(β: Vec<f64>) -> String {
    β.iter()
      .enumerate()
      .map(|(index, value)| format!("β{index} = {value:.5}"))
      .collect::<Vec<String>>()
      .join(",\n")
  }
}

/* impl std::default::Default for RegressionModel<'_> {
  fn default() ->&'_ Self {
    Self {
      x: Array2::<f64>::zeros((1, 1)),
      y: Array2::<f64>::zeros((1, 1)),
      X: Array2::<f64>::zeros((1, 1)),
      Y: Array2::<f64>::zeros((1, 1)),
      β: Array2::<f64>::zeros((1, 1)),
      ε: Array2::<f64>::zeros((1, 1)),
      x_train: Array2::<f64>::zeros((1, 1)).view(),
      x_test: Array2::<f64>::zeros((1, 1)).view(),
      y_train: Array2::<f64>::zeros((1, 1)).view(),
      y_test: Array2::<f64>::zeros((1, 1)).view(),
    }
  }
}
 */
