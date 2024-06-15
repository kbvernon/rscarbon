#' Calibrate radiocarbon measurements
#'
#' @param c14_age a numeric vector of uncalibrated radiocarbon ages
#' @param c14_error a numeric vector of uncalibrated radiocarbon errors
#' @param calibration_tbl a `Calibration` table
#' @param precision a numeric scalar with the desired precision, meaning
#' the value at which smaller densities are set to zero (default 1e-5)
#' @param normalize a logical scalar, if TRUE (default) then each probability
#' distribution is normalized.
#'
#' @return a [CalGrid], strictly an integer vector of row IDs with an
#' attribute containing an external pointer to where the sparse matrix is held
#' in memory.
#'
#' @export
#'
#' @examples
#'
rc_calibrate <- function(
  c14_age,
  c14_error,
  calibration_tbl,
  precision = 1e-5,
  normalize = TRUE
){

  if (!is_calibration(calibration_tbl)){

    cli::cli_abort(c(
      "{.var calibration_tbl} must be a {.cls Calibration} table.",
      "i" = "You supplied a {.cls {class(calibration_tbl)}}."
    ))

  }

  rust_calibrate_independent_ages(
    c14_age = c14_age,
    c14_error = c14_error,
    ybp = calibration_tbl[["ybp"]],
    cal_age = calibration_tbl[["cal_age"]],
    cal_error = calibration_tbl[["cal_error"]],
    cal_name = attr(calibration_tbl, "cal_name"),
    precision = as.numeric(precision),
    sum_to_one = normalize
  )

}

