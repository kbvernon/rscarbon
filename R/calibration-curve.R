#' Radiocarbon calibration curve
#'
#' Build a table of class `Calibration`. Additional methods for converting to
#' class, checking for class, and printing.
#'
#' @param x a `Calibration` table
#' @param ybp a numeric vector of calendar ages
#' @param cal_age a numeric vector of estimated calibration ages
#' @param cal_error a numeric vector of estimated calibration errors
#' @param calibration_name a character scalar, the name of the radiocarbon
#'   calibration curve being used, "<unnamed>" by default
#'
#' @name calibration
#'
#' @return
#' A `Calibration` table that inherits from `tibble`. Has at least the
#' columns `ybp`, `cal_age`, and `cal_error`.
#'
#' @examples
#'
NULL

#' @rdname calibration
#' @export
rc_calibration <- function(
    ybp,
    cal_age,
    cal_error,
    calibration_name = "<unnamed>"
){

  if (missing(ybp)){

    cli::cli_abort(c(
      "Missing calendar age column.",
      "i" = "Please specify with `ybp = '<name_of_ybp_column>'`"
    ))

  }

  if (missing(cal_age)){

    cli::cli_abort(c(
      "Missing estimated calibration age column.",
      "i" = "Please specify with `cal_age = '<name_of_cal_age_column>'`"
    ))

  }

  if (missing(cal_error)){

    cli::cli_abort(c(
      "Missing estimated calibration error column.",
      "i" = "Please specify with `cal_error = '<name_of_cal_error_column>'`"
    ))

  }

  all_integer <- all(c(
    rlang::is_integer(ybp),
    rlang::is_integer(cal_age),
    rlang::is_integer(cal_error)
  ))

  if (!all_integer) {

    cli::cli_abort("`ybp`, `cal_age`, and `cal_error` must be integer vectors.")

  }

  vctrs::new_data_frame(
    list(
      "ybp" = ybp,
      "cal_age" = cal_age,
      "cal_error" = cal_error
    ),
    cal_name = calibration_name,
    class = c("Calibration", "tbl_df", "tbl")
  )

}

#' @rdname calibration
#' @export
is_calibration <- function(x){

  rlang::inherits_any(x, "Calibration")

}

#' @rdname calibration
#' @importFrom pillar tbl_sum
#' @export
tbl_sum.Calibration <- function(x, ...){

  cal_start <- max(x[["ybp"]])
  cal_end <- min(x[["ybp"]])

  c(
    "Calibration curve" = attr(x, "cal_name"),
    "Window" = paste0(cal_start, "-", cal_end, " ybp"),
    NextMethod()
  )

}

#' Convert foreign object to Calibration table
#'
#' @param x object to convert to `Calibration` table
#' @param ybp a character scalar, the name of the column in `x` with
#'   calendar ages
#' @param cal_age a character scalar, the name of the column in `x` with
#'   "estimated" calibration ages
#' @param cal_error a character scalar, the name of the column in `x` with
#'   "estimated" calibration errors
#' @param calibration_name a character scalar, the name of the radiocarbon
#'   calibration curve being used, "<unnamed>" by default
#'
#' @return a `Calibration` table
#' @export
#'
#' @name rc_as_calibration
#'
#' @examples
#'
rc_as_calibration <- function(x, ...){ UseMethod("rc_as_calibration") }

#' @rdname rc_as_calibration
#' @export
rc_as_calibration.data.frame <- function(
    x,
    ybp,
    cal_age,
    cal_error,
    calibration_name = "<unnamed>"
){

  rc_calibration(
    ybp = x[[ybp]],
    cal_age = x[[cal_age]],
    cal_error = x[[cal_error]],
    calibration_name
  )

}
