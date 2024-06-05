use extendr_api::prelude::*;
// use faer::sparse::*;
use rayon::prelude::*;
use statrs::distribution::{Continuous, Normal};

// mod mat_utils;
// use mat_utils::SparseSums;

#[extendr]
fn rust_interpolate_linear(x: &[f64], y: &[f64], xout: &[f64]) -> Vec<f64> {

    xout.iter()
        .map(|&xout| {

            let i = x.partition_point(|&y| y >= xout) - 1usize;

            if xout == x[i] { return y[i]; }

            if xout == x[i + 1] { return y[i + 1]; }

            y[i] + (y[i + 1] - y[i]) * ((xout - x[i]) / (x[i + 1] - x[i]))

        })
        .collect()

}

#[derive(Debug)]
struct CalDate {
    ybp: Vec<f64>,
    density: Vec<f64>
}

#[extendr]
impl CalDate {

    fn new(ybp: Vec<f64>, density: Vec<f64>) -> Self {

        CalDate { ybp, density }

    }

    fn mode(x: Self) -> f64 {

        let i = x.density.iter()
            .enumerate()
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .map(|(index, _)| index)
            .unwrap();

        x.ybp[i]

    }

}

#[extendr]
fn rust_calibrate(
    c14_age: &[f64],
    c14_error: &[f64],
    ybp: &[f64],
    cal_age: &[f64],
    cal_error: &[f64],
    precision: f64,
    normalize: bool,
    cal_name: &str
) -> List {

    let grid: Vec<CalDate> = (c14_age, c14_error)
        .into_par_iter()
        .enumerate()
        .map(|(i, (c14_mu, c14_s))| {

            let (year, mut density): (Vec<f64>, Vec<f64>) = (cal_age, cal_error)
                .into_par_iter()
                .enumerate()
                .filter_map(|(j, (&cal_mu, cal_s))| {

                    let d = dnorm(
                        c14_mu,
                        cal_mu,
                        (c14_s.powi(2i32) + cal_s.powi(2i32)).sqrt()
                    );

                    if d < precision { None } else { Some ((ybp[j], d)) }

                })
                .unzip();

            if normalize { rescale(&mut density) };

            CalDate::new(year, density)

        })
        .collect();

    List::from_values(grid)
        .set_class(vctr_class("CalGrid"))
        .unwrap()

}

fn dnorm(x: &f64, mean: f64, sd: f64) -> f64 {

    let gaussian = Normal::new(mean, sd).unwrap();

    gaussian.pdf(x)

}

fn rescale(x: &mut Vec<f64>) {

    let E = x.iter().sum();

    x.iter_mut().for_each(|y| *y /= E);

}

fn vctr_class(cls: &str) -> [String; 3] {

  let cls = cls.as_str();

  let vct = String::from("vctrs_vctr");

  let lst = String::from("list");

  [cls, vct, lst]

}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rscarbon;
    fn rust_interpolate_linear;
    fn rust_calibrate;
    impl CalDate;
}
