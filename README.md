

# rscarbon

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://www.tidyverse.org/lifecycle/#experimental)

<!-- badges: end -->

The `{rscarbon}` package combines functionality in
[`{rcarbon}`](https://github.com/ahb108/rcarbon) and
[`{Bchron}`](https://github.com/andrewcparnell/Bchron) with the fearless
concurrency of Rust. Right now, it’s highly experimental, and it’s
mainly an opportunity to learn Rust and
[extendr](https://github.com/extendr/extendr). And to get a handle on
this whole radiocarbon dating thing…

I’ve also been digging into
[`{baydem}`](https://github.com/eehh-stanford/baydem), and the use of
Gaussian Mixture Models makes a lot of sense, but I need to think more
about that before trying to implement something that complicated in
Rust.

## Installation

You can - but you shouldn’t! - install the development version of
**rscarbon** from [GitHub](https://github.com/) with:

``` r
# install.packages("pak")
pak::pkg_install("kbvernon/rscarbon")
```

## Radiocarbon calibration

If I were to manually do radiocarbon calibration in R, it would
basically look like this.

``` r
library(rcarbon)
library(rscarbon)
library(tidyverse)

# radiocarbon dataset
c14 <- rcarbon::emedyd |>
  as_tibble() |>
  rename_with(str_to_lower) |> 
  select(cra, error) |> 
  rename("c14_age" = cra, "c14_error" = error) |> 
  slice(1:10)

c14
#> # A tibble: 10 × 2
#>    c14_age c14_error
#>      <int>     <int>
#>  1    9100       140
#>  2    8570       180
#>  3    8775        75
#>  4    8030        65
#>  5    8080        65
#>  6    8205        65
#>  7    7910        60
#>  8    7910        60
#>  9    7980        55
#> 10    7670       100

# calibration curve
rscarbon::intcal20
#> # A tibble: 9,501 × 3
#>      ybp cal_age cal_error
#>    <dbl>   <dbl>     <dbl>
#>  1 55000   50100      1024
#>  2 54980   50081      1018
#>  3 54960   50063      1013
#>  4 54940   50043      1007
#>  5 54920   50027      1003
#>  6 54900   50009       997
#>  7 54880   49992       991
#>  8 54860   49976       987
#>  9 54840   49959       982
#> 10 54820   49943       976
#> # ℹ 9,491 more rows
```

The `c14` table has two important variables:

- `c14_age` = the estimated “age” of the sample derived by radiocarbon
  dating
- `c14_error` = the estimated error or uncertainty, more precisely the
  “measurement” error

The `intcal20` calibration curve has three variables:

- `ybp` = years before present (BP), the “true” calendar years
- `cal_age` = calibrated years BP, the calibration years
- `cal_error` = calibration error

The calibration ages and errors in the calibration “curve”, in effect,
define a set of normal distributions, one for each calendar year BP. So,
the curve is basically a “mixture” of Gaussians.

Note: technically, the variance in these Gaussians is the combined error
from the calibration curve and the c14 sample.

You may have noticed that the calibration curve only defines Gaussians
for an irregular interval of years before present (BP), so the first
thing we need to do is use linear interpolation to expand the
calibration curve so that it includes those intervening years.

But, before we even do that, let’s define a time window for our
analysis, so we don’t perform unnecessary calculations.

``` r
# include fairly large offset to avoid edge effects
time_window <- rev(range(c14[["c14_age"]])) + c(2500, -2500)

intcal20 <- intcal20 |> 
  filter(
    ybp <= time_window[1], 
    ybp >= time_window[2]
  )
```

Now, we can interpolate ages and errors within this restricted curve.

``` r
intcal20 <- with(
  intcal20, {

    # define the set of years that we want to interpolate over    
    xout <- seq(max(ybp), min(ybp), by = -1)
    
    # interpolate the calibration age and error
    int_cal_age <- approx(ybp, cal_age, xout = xout)[["y"]]
    int_cal_error <- approx(ybp, cal_error, xout = xout)[["y"]]
    
    tibble(
      ybp = xout,
      cal_age = int_cal_age,
      cal_error = int_cal_error
    )
    
  }
  
)

intcal20
#> # A tibble: 6,431 × 3
#>      ybp cal_age cal_error
#>    <dbl>   <dbl>     <dbl>
#>  1 11600  10018       24  
#>  2 11599  10017.      24.2
#>  3 11598  10015.      24.4
#>  4 11597  10014.      24.6
#>  5 11596  10012.      24.8
#>  6 11595  10011       25  
#>  7 11594  10011.      24.8
#>  8 11593  10011.      24.6
#>  9 11592  10010.      24.4
#> 10 11591  10010.      24.2
#> # ℹ 6,421 more rows
```

With this expanded curve, we can now estimate the probability density
that a c14 age was actually produced in any given calendar year BP.

A couple of things to note here:

1.  The closer the `c14_age` gets to the `cal_age` for a specific
    calendar year or `ybp`, the greater its probability density is for
    that `ybp`.  
2.  A threshold `eps` is often applied to the density estimates. And
    density `d < eps` is set to zero. The standard is `eps = 1e-5`.  
3.  The vector of densities is often normalized, too: `d/sum(d)`.

``` r
# this is a smidge hard to read because of the nested map calls
get_density <- function(c14_age, c14_error, calibration_curve){
  
  d <- purrr::pmap_dbl(
    calibration_curve, 
    \(ybp, cal_age, cal_error){
      
      dnorm(
        c14_age, 
        mean = cal_age, 
        sd = sqrt(c14_error^2 + cal_error^2)
      )
      
    })
  
  # set small densities to zero
  d[d < 1e-5] <- 0
  
  # normalize
  d/sum(d)

}

c14 <- c14 |> 
  mutate(
    pdf = purrr::map2(
      c14_age, 
      c14_error, 
      \(x, y) { get_density(x, y, intcal20) }
    )
  )

c14
#> # A tibble: 10 × 3
#>    c14_age c14_error pdf          
#>      <int>     <int> <list>       
#>  1    9100       140 <dbl [6,431]>
#>  2    8570       180 <dbl [6,431]>
#>  3    8775        75 <dbl [6,431]>
#>  4    8030        65 <dbl [6,431]>
#>  5    8080        65 <dbl [6,431]>
#>  6    8205        65 <dbl [6,431]>
#>  7    7910        60 <dbl [6,431]>
#>  8    7910        60 <dbl [6,431]>
#>  9    7980        55 <dbl [6,431]>
#> 10    7670       100 <dbl [6,431]>
```

This list column is a profoundly inefficient way to store these vectors,
as they are loaded down with zeroes.

``` r
map_int(c14[["pdf"]], \(x){ sum(x==0) }) |> mean()
#> [1] 5612.1
```

So, it makes sense to drop the zeroes and just keep track of the
non-zero values and their index (or ybp) in the vector of densities - in
a word, to store them in a sparse matrix, with a column for each year BP
and a row for each c14 sample.

``` r
library(Matrix)

M <- Matrix(
  do.call("rbind", c14[["pdf"]]), 
  sparse = TRUE
)

M[1:5, 1:5]
#> 5 x 5 sparse Matrix of class "dgCMatrix"
#>               
#> [1,] . . . . .
#> [2,] . . . . .
#> [3,] . . . . .
#> [4,] . . . . .
#> [5,] . . . . .
```

This is kinda sorta what **rcarbon** does by default. Unfortunately,
they do not take advantage of the base R `{Matrix}` package, which would
make all of their calculations much faster and more efficient.

## Summed probability distribution

Funny enough, this is just:

``` r
D <- colSums(M)
```

Here’s what that looks like:

``` r
my_spd <- tibble(
  ybp = intcal20[["ybp"]],
  spd = D
)

ggplot(my_spd) +
  geom_line(aes(ybp, spd)) +
  labs(x = "Year [BP]", y = "Probability Density") +
  theme_bw(12) +
  theme(panel.grid.minor = element_blank())
```

<img src="man/figures/README-unnamed-chunk-8-1.png"
style="width:60.0%" />

## Sparse Vectors?

If we wanted to stay in a table environment, we could also try
converting each vector to a `sparseVector`.

``` r
to_sparse_vector <- function(x){
  
  i <- which(x > 0)
  
  Matrix::sparseVector(x[i], i, length = length(x))
  
}

c14 <- c14 |> mutate(pdf = purrr::map(pdf, to_sparse_vector))
```

I guess one of the nice things about using the `sparseVector` in a
table, is that you can build grouped SPDs using `{dplyr}` and
`group_by() |> summarize()`. This requires some work though to build a
sparse matrix out of each subset.

``` r
sum_sparse_vectors <- function(x){
  
  idx <- purrr::map(x, \(z){ z@i })
  values <- purrr::map(x, \(z){ z@x })
  
  nr <- length(x)
  nc <- length(x[[1]])
  
  M <- Matrix::sparseMatrix(
    i = rep(1:nr, lengths(idx)),
    j = unlist(idx),
    x = unlist(values),
    dims = c(nr, nc)
  )
  
  list(Matrix::colSums(M))
  
}

c14 |> 
  mutate(grp = sample(1:2, size = 10, replace = TRUE)) |> 
  group_by(grp) |> 
  summarize(pdf = sum_sparse_vectors(pdf)) |> 
  ungroup()
#> # A tibble: 2 × 2
#>     grp pdf          
#>   <int> <list>       
#> 1     1 <dbl [6,431]>
#> 2     2 <dbl [6,431]>
```

## Benchmarks

``` r
library(Bchron)
library(rcarbon)
library(rscarbon)

ages <- c14[["c14_age"]]
errors <- c14[["c14_error"]]

bench::mark(
  bchron = Bchron::BchronCalibrate(ages, errors),
  rcarbon = rcarbon::calibrate(ages, errors, verbose = FALSE, ncores = 4),
  rscarbon = rscarbon::rc_calibrate(ages, errors),
  iterations = 100,
  check = FALSE
) |> select(expression, median, 'itr/sec')
```

This is probably not a fair comparison, but the results do look
promising.
