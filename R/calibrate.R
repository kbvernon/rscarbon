
#' Title
#'
#' @param age a numeric vector of uncalibrated radiocarbon ages
#' @param error a numeric vector of uncalibrated radiocarbon errors
#' @param time_range a vector of length two with start and end dates
#' (note that start > end)
#' @param calibration a scalar character vector with the name of the calibration
#' curve
#' @param precision a scalar numeric vector with the desired precision, meaning
#' the value at which smaller densities are set to zero
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
  calibration = "intcal20",
  precision = 1e-5
){

  all_calibrations <- c(
    "intcal13", "intcal13nhpine16", "intcal20", "marine13", "marine20",
    "normal", "shcal13", "shcal13shkauri16", "shcal20"
  )

  if (!calibration %in% all_calibrations){

    cli::cli_abort(
      "{calibration} is not a recognized calibration curve.",
      "i" = "Available curves include {all_calibrations}."
    )

  }

  path_to_calibration <- system.file(
    "extdata", paste0(calibration, ".14c"),
    package = "rcarbon"
  )

  rust_calibrate(
    age = as.numeric(age),
    error = as.numeric(error),
    start = as.integer(time_range[2]),
    end = as.integer(time_range[1]),
    precision = as.numeric(precision),
    path_to_calibration
  )

}
