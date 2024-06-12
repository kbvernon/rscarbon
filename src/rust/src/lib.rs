use extendr_api::prelude::*;
use rayon::prelude::*;
use statrs::distribution::{Continuous, Normal};

#[derive(Debug)]
struct CalDate {
    ybp: Vec<i32>,
    density: Vec<f64>
}

#[extendr]
impl CalDate { }

impl CalDate {

    fn new(ybp: Vec<i32>, density: Vec<f64>) -> Self {

        CalDate { ybp, density }

    }

    fn mode(&self) -> i32 {

        let i = self.density.iter()
            .enumerate()
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .map(|(index, _)| index)
            .unwrap();

        self.ybp[i]

    }

    fn interpolate(&mut self) {

        let start = *self.ybp.iter().max().unwrap();
        let end = *self.ybp.iter().min().unwrap();

        let xout: Vec<i32> = (end..=start).rev().collect();

        (self.ybp, self.density) = xout.into_iter()
            .map(|u| {

                let i = self.ybp.partition_point(|&v| v >= u) - 1usize;

                if u == self.ybp[i] { return (u, self.density[i]) }

                if u == self.ybp[i + 1] { return (u, self.density[i + 1]) }

                let x1 = f64::from(self.ybp[i]);
                let x2 = f64::from(self.ybp[i + 1]);

                let y1 = self.density[i];
                let y2 = self.density[i + 1];

                let yhat = y1 + (y2 - y1) * ((f64::from(u) - x1) / (x2 - x1));

                (u, yhat)

            })
            .unzip();

    }

    fn normalize(&mut self) {

        let total: f64 = self.density.iter().sum();

        self.density.iter_mut().for_each(|y| *y /= total);

    }

}

#[extendr]
fn rust_caldate_mode(x: ExternalPtr<CalDate>) -> i32 { x.mode() }

#[extendr]
fn rust_collect(x: List) -> RMatrix<f64> {

    let w = x.get_attrib("window").unwrap().as_integer_vector().unwrap();

    let start = w[0];
    let end = w[1];

    let nrow = x.len();
    let ncol: usize = (start - end + 1).try_into().unwrap();

    let mut matrix = RMatrix::new_matrix(nrow, ncol, |_, _| 0.0);

    for (i, u) in x.values().enumerate() {

        let cal_date: ExternalPtr<CalDate> = u.try_into().unwrap();

        let iterator = cal_date.ybp.iter().zip(cal_date.density.iter());

        for (&ybp, &density) in iterator {

            let j: usize = (start - ybp).try_into().unwrap();

            matrix[[i, j]] = density;

        }

    }

    matrix

}

#[extendr]
fn rust_calibrate_independent_ages(
    c14_age: &[i32],
    c14_error: &[i32],
    ybp: &[i32],
    cal_age: &[i32],
    cal_error: &[i32],
    precision: f64,
    sum_to_one: bool,
    cal_name: &str
) -> List {

    let grid: Vec<CalDate> = (c14_age, c14_error)
        .into_par_iter()
        .map(|(&x, &y)| {

            calibrate_single_age(
                x,
                y,
                ybp,
                cal_age,
                cal_error,
                precision,
                sum_to_one
            )

        })
        .collect();

    let mut v_start = vec![0; grid.len()];
    let mut v_end = vec![0; grid.len()];

    for (i, y) in grid.iter().enumerate() {

        v_start[i] = *y.ybp.first().unwrap();
        v_end[i] = *y.ybp.last().unwrap();

    }

    let start = v_start.iter().max().unwrap();
    let end = v_end.iter().min().unwrap();

    let current_window = Integers::from_values(vec![start, end]);

    let mut list = List::from_values(grid);

    list.set_class(vctr_class("CalGrid"))
        .unwrap()
        .set_attrib("cal_name", cal_name)
        .unwrap()
        .set_attrib("normalize", sum_to_one)
        .unwrap()
        .set_attrib("window", current_window)
        .unwrap();

    list

}

fn calibrate_single_age(
    c14_age: i32,
    c14_error: i32,
    ybp: &[i32],
    cal_age: &[i32],
    cal_error: &[i32],
    precision: f64,
    sum_to_one: bool
) -> CalDate {

    let c14_mu = f64::from(c14_age);
    let c14_sd2 = f64::from(c14_error).powi(2i32);

    let (year, density): (Vec<i32>, Vec<f64>) = cal_age.iter()
        .zip(cal_error.iter())
        .enumerate()
        .filter_map(|(i, (&u, &v))| {

            let cal_mu = f64::from(u);
            let cal_sd2 = f64::from(v).powi(2i32);

            let total_error = (c14_sd2 + cal_sd2).sqrt();

            let d = dnorm(c14_mu, cal_mu, total_error);

            if d < precision { None } else { Some ((ybp[i], d)) }

        })
        .unzip();

    let mut new_date = CalDate::new(year, density);

    new_date.interpolate();

    if sum_to_one { new_date.normalize() }

    new_date

}

fn dnorm(x: f64, mean: f64, sd: f64) -> f64 {

    let gaussian = Normal::new(mean, sd).unwrap();

    gaussian.pdf(x)

}

fn vctr_class(cls: &str) -> [String; 3] {

    let cls = cls.into();

    let vct = String::from("vctrs_vctr");

    let lst = String::from("list");

    [cls, vct, lst]

}

#[extendr]
fn rust_spd(x: List) -> RMatrix<f64> {

    let w = x.get_attrib("window").unwrap().as_integer_vector().unwrap();

    let start = w[0];
    let end = w[1];

    let nrow: usize = (start - end + 1).try_into().unwrap();

    let mut matrix = RMatrix::new_matrix(nrow, 2, |_, _| 0.0);

    for (i, u) in (end..=start).rev().enumerate() {

        matrix[[i, 0usize]] = u as f64;

    }

    for u in x.values() {

        let cal_date: ExternalPtr<CalDate> = u.try_into().unwrap();

        let iterator = cal_date.ybp.iter().zip(cal_date.density.iter());

        for (&ybp, &density) in iterator {

            let i: usize = (start - ybp).try_into().unwrap();

            matrix[[i, 1usize]] += density;

        }

    }

    let dim_names = list!(().into_robj(), &["ybp", "prob_dens"]);

    matrix.set_attrib("dimnames", dim_names).unwrap();

    matrix

}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rscarbon;
    fn rust_calibrate_independent_ages;
    impl CalDate;
    fn rust_caldate_mode;
    fn rust_collect;
    fn rust_spd;
}

