% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/calibration-curve.R
\name{rc_as_calibration}
\alias{rc_as_calibration}
\alias{rc_as_calibration.data.frame}
\title{Convert foreign object to Calibration table}
\usage{
rc_as_calibration(x, ...)

\method{rc_as_calibration}{data.frame}(x, ybp, cal_age, cal_error, calibration_name = "<unnamed>")
}
\arguments{
\item{x}{object to convert to \code{Calibration} table}

\item{ybp}{a character scalar, the name of the column in \code{x} with
calendar ages}

\item{cal_age}{a character scalar, the name of the column in \code{x} with
"estimated" calibration ages}

\item{cal_error}{a character scalar, the name of the column in \code{x} with
"estimated" calibration errors}

\item{calibration_name}{a character scalar, the name of the radiocarbon
calibration curve being used, "\if{html}{\out{<unnamed>}}" by default}
}
\value{
a \code{Calibration} table
}
\description{
Convert foreign object to Calibration table
}
