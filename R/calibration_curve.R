
#' Radiocarbon calibration curve
#'
#' Build a table of class [Calibration]. Additional methods for converting to
#' class, checking for class, and printing.
#'
#' @param x
#'  * for [is_calibration()] and [print()] a [Calibration] table
#'  * for [as_calibration()] a [tbl] or [data.frame] to convert to [Calibration]
#' @param cal_age
#'  * for [calibration()] a numeric vector of calendar ages
#'  * for [as_calibration()] a character scalar, the name of the column in [x]
#'  with calendar ages
#' @param est_age
#'  * for [calibration()] a numeric vector of estimated ages
#'  * for [as_calibration()] a character scalar, the name of the column in [x]
#'  with estimated ages
#' @param est_error
#'  * for [calibration()] a numeric vector of estimated errors
#'  * for [as_calibration()] a character scalar, the name of the column in [x]
#'  with estimated errors
#' @param calibration_name a character scalar, the name of the radiocarbon
#' calibration curve being used
#' @param n a numeric scalar, the number of rows in [data.frame] to print
#'
#' @name calibration
#'
#' @return
#' A [Calibration] table that inherits from [tibble] and [data.frame]. Has three
#' columns: cal_age, est_age, est_error.
#'
#' @examples
#'
NULL

#' @rdname calibration
#' @export
calibration <- function(
    cal_age,
    est_age,
    est_error,
    calibration_name
){

  vctrs::new_data_frame(
    list(
      "cal_age" = cal_age,
      "est_age" = est_age,
      "est_error" = est_error
    ),
    cal_name = calibration_name,
    class = c("Calibration", "tbl_df", "tbl")
  )

}

#' @rdname calibration
#' @export
as_calibration <- function(
    x,
    calendar_age,
    calibration_age,
    calibration_error,
    cal_name = "<unknown>"
){ UseMethod("as_calibration") }

#' @rdname calibration
#' @export
as_calibration.tbl <- function(x, ...){

  lst <- rlang::list2(...)

  need_names <- c("cal_age", "est_age", "est_error")

  if (!all(need_names %in% names(lst))){

    cli::cli_abort(c(
      "Need column names in {.var x} for {need_names}",
      "i" = "Missing {setdiff(need_names, names(lst))}."
    ))

  }

  calibration(
    cal_age = x[[cal_age]],
    est_age = x[[est_age]],
    est_error = x[[est_error]],
    cal_name
  )

}

#' @rdname calibration
#' @export
as_calibration.data.frame <- function(x, ...){

  lst <- rlang::list2(...)

  need_names <- c("cal_age", "est_age", "est_error")

  if (!all(need_names %in% names(lst))){

    cli::cli_abort(c(
      "Need column names in {.var x} for {need_names}",
      "i" = "Missing {setdiff(need_names, names(lst))}."
    ))

  }

  cal_age <- lst[["cal_age"]]
  est_age <- lst[["est_age"]]
  est_error <- lst[["est_error"]]

  calibration(
    cal_age = x[[cal_age]],
    est_age = x[[est_age]],
    est_error = x[[est_error]],
    cal_name
  )

}

#' @rdname calibration
#' @export
is_calibration <- function(x){

  rlang::inherits_any(x, "Calibration")

}

#' @rdname calibration
#' @export
print.Calibration <- function(x, ...){

  cal_start <- max(x[["cal_age"]])
  cal_end <- min(x[["cal_age"]])

  cli::cli_h2("Radiocarbon calibration curve")
  cli::cli_text("Curve: {attr(x, 'cal_name')}")
  cli::cli_text("Window: {cal_start}-{cal_end} ybp")
  cli::cli_text("")

  NextMethod()

}

