## code to prepare `intcal20` dataset goes here

library(readr)

calbs <- readr::read_csv(
  "https://raw.githubusercontent.com/ahb108/rcarbon/master/inst/extdata/intcal20.14c",
  skip = 11,
  col_select = 1:3,
  col_names = c("calbp", "c14bp", "tau"),
  col_types = "d"
)

usethis::use_data(calbs, overwrite = TRUE)
