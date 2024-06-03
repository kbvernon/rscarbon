
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
calibrate <- function(
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

  ptr <- rs_calibrate(
    c14_age = as.numeric(c14_age),
    c14_error = as.numeric(c14_error),
    cal_age = calibration_tbl[["cal_age"]],
    est_age = calibration_tbl[["est_age"]],
    est_error = calibration_tbl[["est_error"]],
    precision = as.numeric(precision),
    normalize = normalize
  )

  vctrs::new_vctr(
    1:length(c14_age),
    extptr = ptr,
    class = "CalGrid"
  )

}

