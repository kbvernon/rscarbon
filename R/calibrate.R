
#' Title
#'
#' @param age a numeric vector of uncalibrated radiocarbon ages
#' @param error a numeric vector of uncalibrated radiocarbon errors
#' @param time_range a vector of length two with start and end dates
#' (note that start > end)
#' @param precision a scalar numeric vector with the desired precision, meaning
#' the value at which smaller densities are set to zero
#' @param calibration an object of class `Calibration` built using `read_14c()`
#'
#' @return a list of probability density distributions, with length equal to
#' `length(age)`
#' @export
#'
#' @examples
calibrate <- function(
  age,
  error,
  time_range = c(55000, 0),
  precision = 1e-5,
  calibration
){

  rust_calibrate(
    age = as.numeric(age),
    error = as.numeric(error),
    start = as.integer(time_range[2]),
    end = as.integer(time_range[1]),
    precision = as.numeric(precision),
    calibration = calibration
  )

}
