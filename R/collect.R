
#' Move data into local R object
#'
#' Currently, returns a dense matrix, but will hopefully convert this to a
#' sparse matrix at some point.
#'
#' @param x a `CalGrid`
#'
#' @return
#' @export
#'
#' @examples
rc_collect <- function(x){

  if (!rlang::inherits_any(x, "CalGrid")){

    cli::cli_abort(c(
      "{.var x} must be a {.cls CalGrid} vector.",
      "i" = "You supplied a {.cls {class(x)}}."
    ))

  }

  rust_collect(x)

}
