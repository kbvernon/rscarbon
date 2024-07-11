
calibrated_grid <- function(
    ptr,
    ids = integer(),
    nrow = integer(),
    ncol = integer(),
    normalize = logical()
){

  if (!rlang::inherits_any(ptr, "externalptr")){

    cli::cli_abort(c(
      "{.var ptr} must be an {.cls externalptr}.",
      "i" = "You supplied a {.cls {class(ptr)}}."
    ))

  }

  if (!rlang::is_integer(ids)){

    cli::cli_abort(c(
      "{.var ids} must be an {.cls integer} vector.",
      "i" = "You supplied a {.cls {class(ids)}}."
    ))

  }

  if (!rlang::is_integer(nrow)){

    cli::cli_abort(c(
      "{.var nrow} must be an {.cls integer} scalar.",
      "i" = "You supplied a {.cls {class(nrow)}}."
    ))

  }

  if (!rlang::is_integer(ncol)){

    cli::cli_abort(c(
      "{.var ncol} must be an {.cls integer} scalar.",
      "i" = "You supplied a {.cls {class(ncol)}}."
    ))

  }

  if (!rlang::is_logical(normalize)){

    cli::cli_abort(c(
      "{.var normalize} must be an {.cls logical} scalar.",
      "i" = "You supplied a {.cls {class(normalize)}}."
    ))

  }

  vctrs::new_vctr(
    .data = ids,
    grid = ptr,
    normalize = normalize,
    class = "CalGrid"
  )

}

format.CalGrid <- function(x, ...){

  purrr::map_int(x, rust_caldate_mode)

}

caldate_tbl_formatter <- function(x){

  ret <- sprintf(
    "%d%s",
    rust_mode(x),
    pillar::style_subtle("YBP")
  )

  format(ret, justify = "right")

}

pillar_shaft.CalGrid <- function(x, ...){

  out <- format(x, formatter = caldate_tbl_formatter)
  pillar::new_pillar_shaft_simple(out, align = "right")

}



print.CalDate <- function(x, ...){ cat(rust_caldate_mode(x)) }
