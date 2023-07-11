from dataclasses import dataclass
from pprint import pp, pprint

import numpy as np
from sklearn.linear_model import LinearRegression
from sklearn.metrics import r2_score, mean_squared_error, mean_absolute_error
from sklearn.model_selection import train_test_split

from sklearn.preprocessing import PolynomialFeatures
from sklearn.pipeline import make_pipeline


@dataclass
class RegressionResult:
  """Represents the results of a regression model."""

  MSE: float = 0.0
  """Represents the results of a regression model."""

  prediction_error: float = 0.0
  """Percentage of error of the regression model."""

  determination: float = 0.0
  """Performance of the regression model. Also called R2 score."""


def run_linear_regression(
    X: list, Y: list, test_size: float = 0.2) -> RegressionResult:
  # Create the data for train and test
  X_train, X_test, Y_train, Y_test = train_test_split(
    X, Y, test_size=test_size, random_state=0)

  # Create the linear regression
  linear_regresssion = LinearRegression()

  # Train the pipeline
  linear_regresssion.fit(X_train, Y_train)

  # Calculate the prediction over the test dataset
  predictions = linear_regresssion.predict(X_test)

  # Calculate the squared error
  squared_error = np.sqrt(mean_squared_error(Y_test, predictions))

  result = RegressionResult()
  result.MSE = squared_error

  #print(f"Mean Error: {squared_error:.3f}  ({((squared_error / np.mean(predictions)) * 100):.3f} %)")

  # Calculate the percentage of the prediction error
  result.prediction_error = (squared_error / np.mean(predictions)) * 100

  # Calculate the model determination
  score = linear_regresssion.score(X_train, Y_train)
  result.determination = score

  #print(f"Model determination: {score:.3f}")

  return result


def run_polynomial_regression(X, Y, test_size=0.2):
  # Create the data for train and test
  X_train, X_test, Y_train, Y_test = train_test_split(
    X, Y, test_size=test_size, random_state=0)

  # Create the pipeline of estimators
  estimators_pipeline = make_pipeline(
    PolynomialFeatures(2), LinearRegression())

  # Train the pipeline
  estimators_pipeline.fit(X_train, Y_train)

  # Calculate the predictions for the test data
  predictions = estimators_pipeline.predict(X_test)

  result = RegressionResult()

  # Calculate the squared error
  squared_error = np.sqrt(mean_squared_error(Y_test, predictions))

  result.MSE = squared_error

  result.prediction_error = (squared_error / np.mean(predictions)) * 100 
  
  # Calculate the model determination
  score = estimators_pipeline.score(X_train, Y_train)

  result.determination = score

  return result


def format_results(
    result_name: str, results_data: list, result: RegressionResult) -> list:
  """Adds and gives format to the given result object and appends it to the given results list."""
  results_data.append(
    [
    result_name, f"{result.MSE:.2f} ({result.prediction_error:.2f} %)",
    f"{result.determination:.2f}"
    ])
  return results_data
