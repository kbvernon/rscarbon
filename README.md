

# rscarbon

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://www.tidyverse.org/lifecycle/#experimental)

<!-- badges: end -->

The `{rscarbon}` package provides tools for calibrating and summarizing
radiocarbon dates. It’s mostly an R wrapper around Rust code, hence the
**rs** in “rscarbon”. Right now, it’s highly experimental, mainly just
serving as an opportunity to learn Rust and
[extendr](https://github.com/extendr/extendr) and to get a handle on
this whole radiocarbon dating thing…

## Installation

You can - but you shouldn’t! - install the development version of
**rscarbon** from [GitHub](https://github.com/) with:

``` r
# install.packages("pak")
pak::pkg_install("kbvernon/rscarbon")
```

## Motivation

In archaeology, we have this general rule-of-thumb that more people
means more stuff - more houses, more cars, more washers, more microwave
ovens, or, anyway, more of whatever it is that people in a particular
time and place rely on to get by. It follows that if you radiocarbon
date a random sample of all that stuff, you can use the count of
radiocarbon dates themselves as a proxy for population levels at
different times and places. Basically:

> More People **⇾** More Archaeology **⇾** More Radiocarbon Dates

This sort of inference is sometimes referred to as “dates as data”. This
is a perfectly reasonable and intuitive inference, but of course, once
you start poking at it, you realize there are all sorts of issues.
\[elaborate on this in vignette\]

## Other R packages

Here are some of the more popular R packages that provide tools for
working with radiocarbon dates:

- [`{IntCal}`](https://github.com/cran/IntCal), this is a read-only
  mirror

- [`{rcarbon}`](https://github.com/ahb108/rcarbon)

- [`{Bchron}`](https://github.com/andrewcparnell/Bchron)

- [`{baydem}`](https://github.com/eehh-stanford/baydem), implements
  methods proposed in \[@price_etal2021\]. Has a really unfortunate API
  reminiscent of GRASS. It’ll feel pretty weird to most piRates.

- [`{c14}`](https://github.com/joeroe/c14), hands-down the best API out
  there, but not currently on CRAN, mostly a tidy-wrapper around the
  other packages on this list

- [`{carbon14}`](https://github.com/paleolimbot/carbon14), because of
  course Dewey Dunnington would have written such a package, being both
  a paleoecologist and a statistical programming wizard. Unfortunately,
  he seems to have let this one go by the wayside.

- [`{carbondate}`](https://github.com/TJHeaton/carbondate), implements
  methods proposed in \[@heaton2022\]. Near as I can tell, this one
  offers a non-parametric form of the model used in **baydem**, but it’s
  built almost entirely in C++, so it’s extremely lightweight in terms
  of dependencies.

## Benchmarks

These do not represent a comprehensive set of tests, but they are
suggestive.

``` r
library(Bchron)
library(bench)
library(carbondate)
library(IntCal)
library(rcarbon)
library(rscarbon)
library(tidyverse)

data(emedyd)

set.seed(1701)

c14 <- emedyd |>
  as_tibble() |>
  rename_with(str_to_lower) |>
  select(cra, error) |>
  rename("age" = cra, "error" = error) |> 
  slice_sample(n = 10)

rm(emedyd)
```

### Radiocarbon calibration

``` r
# bchron function uses print() a lot
bchron_calibrate <- purrr::quietly(Bchron::BchronCalibrate)

# intcal function isn't vectorized
intcal_calibrate <- function(ages, errors){

  purrr::map2(
    ages,
    errors,
    \(x, y) IntCal::caldist(x, y, yrsteps = 1, threshold = 1e-5)
  )

}

rcarbon_calibrate <- function(ages, errors){
  
  rcarbon::calibrate(ages, errors, verbose = FALSE)
  
}

rscarbon_calibrate <- function(ages, errors){
  
  rscarbon::rc_calibrate(ages, errors, calibration_tbl = rscarbon::intcal20)
  
}

bench::mark(
  bchron = bchron_calibrate(c14[["age"]], c14[["error"]]),
  intcal = intcal_calibrate(c14[["age"]], c14[["error"]]),
  rcarbon = rcarbon_calibrate(c14[["age"]], c14[["error"]]),
  rscarbon = rscarbon_calibrate(c14[["age"]], c14[["error"]]),
  check = FALSE,
  relative = TRUE
) |> select(expression:mem_alloc) 
#> # A tibble: 4 × 5
#>   expression   min median `itr/sec` mem_alloc
#>   <bch:expr> <dbl>  <dbl>     <dbl>     <dbl>
#> 1 bchron      412.   386.      1.63      42.4
#> 2 intcal      493.   442.      1         83.3
#> 3 rcarbon     360.   332.      1.92      74.3
#> 4 rscarbon      1      1     619.         1
```

Note: I’m aware that **rcarbon** has support for parallel processing,
but it performs considerably worse on small samples (like 20x slower).

### Summed Probability Distribution

There are some fundamental issues with using the Summed Probability
Distribution (SPD) approach, but putting that aside…

``` r
rcarbon_calgrid <- rcarbon::calibrate(
  c14[["age"]], 
  c14[["error"]],
  calMatrix = TRUE,
  verbose = FALSE
)

rcarbon_spd <- function(x){
  
  rcarbon::spd(x, timeRange = c(11500, 8500), verbose = FALSE)
  
}

rscarbon_calgrid <- rscarbon::rc_calibrate(
  c14[["age"]], 
  c14[["error"]],
  calibration_tbl = rscarbon::intcal20
)

rscarbon_spd <- function(x){ rscarbon::rc_spd(x) }

bench::mark(
  rcarbon = rcarbon_spd(rcarbon_calgrid),
  rscarbon = rscarbon_spd(rscarbon_calgrid),
  check = FALSE,
  relative = TRUE
) |> select(expression:mem_alloc) 
#> # A tibble: 2 × 5
#>   expression   min median `itr/sec` mem_alloc
#>   <bch:expr> <dbl>  <dbl>     <dbl>     <dbl>
#> 1 rcarbon     215.   105.        1       66.1
#> 2 rscarbon      1      1       103.       1
```
