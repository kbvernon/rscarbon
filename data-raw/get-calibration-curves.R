## code to prepare 14c calibration curve datasets

get_14c <- function(x){

  res <- readr::read_csv(
    file.path("https://intcal.org/", "curves", x),
    skip = 11,
    col_select = 1:3,
    col_names = c("cal_age", "est_age", "est_error"),
    col_types = "d"
  )

  vctrs::new_data_frame(
    res,
    cal_name = tools::file_path_sans_ext(x),
    class = c("Calibration", "tbl_df", "tbl")
  )

}

intcal13 <- get_14c("intcal13.14c")
intcal20 <- get_14c("intcal20.14c")
marine13 <- get_14c("marine13.14c")
marine20 <- get_14c("marine20.14c")
shcal13  <- get_14c("shcal13.14c")
shcal20  <- get_14c("shcal20.14c")

usethis::use_data(
  intcal13,
  intcal20,
  marine13,
  marine20,
  shcal13,
  shcal20,
  overwrite = TRUE
)
