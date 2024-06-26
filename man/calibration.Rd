% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/calibration-curve.R
\name{calibration}
\alias{calibration}
\alias{rc_calibration}
\alias{is_calibration}
\alias{tbl_sum.Calibration}
\title{Radiocarbon calibration curve}
\usage{
rc_calibration(ybp, cal_age, cal_error, calibration_name = "<unnamed>")

is_calibration(x)

\method{tbl_sum}{Calibration}(x, ...)
}
\arguments{
\item{ybp}{a numeric vector of calendar ages}

\item{cal_age}{a numeric vector of estimated calibration ages}

\item{cal_error}{a numeric vector of estimated calibration errors}

\item{calibration_name}{a character scalar, the name of the radiocarbon
calibration curve being used, "\if{html}{\out{<unnamed>}}" by default}

\item{x}{a \code{Calibration} table}
}
\value{
A \code{Calibration} table that inherits from \code{tibble}. Has at least the
columns \code{ybp}, \code{cal_age}, and \code{cal_error}.
}
\description{
Build a table of class \code{Calibration}. Additional methods for converting to
class, checking for class, and printing.
}
