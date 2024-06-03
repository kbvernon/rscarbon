
#' Interpolate Calibration curve
#'
#' Radiocarbon calibration curves usually only estimate true ages at 10-year
#' (or greater) intervals. This function expands the curve to include estimates
#' in all intervening years using linear interpolation. Implemented in Rust for
#' additional speed.
#'
#' @param x a [Calibration] table
#' @param xout years in which to estimate true ages. If NULL (default), then all
#' years in [seq(max(x[["cal_age"]]), min(x[["cal_age"]]), by = -1)] are used.
#'
#' @return
#' A [Calibration] table that inherits from [tibble] and [data.frame]. Has three
#' columns: cal_age, est_age, est_error.
#'
#' @export
#'
#' @examples
interpolate <- function(x, xout = NULL){

  if (!is_calibration(x)){

    cli::cli_abort(c(
      "{.var x} must be a {.cls Calibration} table.",
      "i" = "You supplied a {.cls {class(x)}}."
    ))

  }

  if (is.null(xout)){

    xout <- seq(max(x[["cal_age"]]), min(x[["cal_age"]]), by = -1)

  }

  calibration(
    cal_age = xout,
    est_age = rs_interpolate_linear(x[["cal_age"]], x[["est_age"]], xout),
    est_error = rs_interpolate_linear(x[["cal_age"]], x[["est_error"]], xout),
    calibration_name = attr(x, "cal_name")
  )

}
