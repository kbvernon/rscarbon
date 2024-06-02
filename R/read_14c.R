
read_14c <- function(x = "intcal20"){

  all_calibrations <- c(
    "intcal13", "intcal13nhpine16", "intcal20", "marine13", "marine20",
    "normal", "shcal13", "shcal13shkauri16", "shcal20"
  )

  if (!x %in% all_calibrations){

    cli::cli_abort(
      "{x} is not a recognized calibration curve.",
      "i" = "Available curves include {all_calibrations}."
    )

  }

  path <- system.file("extdata", paste0(x, ".14c"), package = "rcarbon")

  Calibration$read_14c(path)

}
