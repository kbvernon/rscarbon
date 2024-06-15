#' Sum calibrated radiocarbon probability distributions
#'
#' @param x a `CalGrid` vector
#' @param normalize a logical scalar, if TRUE (default) then each probability
#' distribution is normalized.
#'
#' @return a `matrix` with two columns `ybp` and `prob_dens`
#' @export
#'
#' @examples
#'
rc_spd <- function(x, normalize = TRUE){

  if (!rlang::inherits_any(x, "CalGrid")){

    cli::cli_abort(c(
      "{.var x} must be a {.cls CalGrid} vector.",
      "i" = "You supplied a {.cls {class(x)}}."
    ))

  }

  rust_spd(x, sum_to_one = normalize)

}
