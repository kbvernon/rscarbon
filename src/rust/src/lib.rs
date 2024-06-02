use csv::ReaderBuilder;
use extendr_api::prelude::*;
use faer::sparse::*;
use rayon::prelude::*;
use statrs::distribution::{Continuous, Normal};

#[derive(Debug)]
struct Calibration {
    pub calbp: Vec<f64>,
    pub c14bp: Vec<f64>,
    pub tau: Vec<f64>
}

#[extendr]
impl Calibration {

    fn read_14c(path_to_calibration: &str) -> Self {

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .comment(Some(b'#'))
            .from_path(path_to_calibration)
            .unwrap();

        let mut calbp: Vec<f64> = Vec::new();
        let mut c14bp: Vec<f64> = Vec::new();
        let mut tau: Vec<f64> = Vec::new();

        for result in rdr.records() {

            let record = result.unwrap();

            calbp.push(record[0].parse::<f64>().unwrap());
            c14bp.push(record[1].parse::<f64>().unwrap());
            tau.push(record[2].parse::<f64>().unwrap());

        }

        Self { calbp, c14bp, tau }

    }

}

#[extendr]
fn rust_calibrate(
    age: &[f64],
    error: &[f64],
    start: i32,
    end: i32,
    precision: f64,
    calbs: Dataframe<()>,
) -> ExternalPtr<SparseColMat<usize, f64>> {
    let calbp: &[f64] = calbs.dollar("calbp").unwrap().try_into().unwrap();
    let c14bp: &[f64] = calbs.dollar("c14bp").unwrap().try_into().unwrap();
    let tau: &[f64] = calbs.dollar("tau").unwrap().try_into().unwrap();

    let c14out: Vec<f64> = (start..end)
        .step_by(1)
        .rev()
        .map(|x| f64::from(x))
        .collect();

    let c14pd = calibrate_bp14c(
        &age,
        &error,
        &interpolate_linear(calbp, c14bp, &c14out),
        &interpolate_linear(calbp, tau, &c14out),
        precision,
    );

    ExternalPtr::new(c14pd)
}

fn calibrate_bp14c(
    age: &[f64],
    error: &[f64],
    mu: &[f64],
    tau: &[f64],
    precision: f64,
) -> SparseColMat<usize, f64> {
    let res = age
        .into_par_iter()
        .zip(error.into_par_iter())
        .enumerate()
        .flat_map(|(i, (&s_mean, &s_error))| {
            mu.iter()
                .zip(tau.iter())
                .enumerate()
                .filter_map(|(j, (&c_mean, &c_error))| {
                    let n = Normal::new(c_mean, (s_error.powi(2i32) + c_error.powi(2i32)).sqrt())
                        .unwrap();

                    let d = n.pdf(s_mean);

                    if d < precision {
                        None
                    } else {
                        Some((i, j, d))
                    }
                })
                .collect::<Vec<(usize, usize, f64)>>()
        })
        .collect::<Vec<(usize, usize, f64)>>();

    SparseColMat::<usize, f64>::try_new_from_triplets(age.len(), mu.len(), &res).unwrap()
}

fn interpolate_linear(x: &[f64], y: &[f64], xout: &[f64]) -> Vec<f64> {
    xout.iter()
        .map(|&xout| {
            let i = x.partition_point(|&y| y >= xout) - 1usize;

            if xout == x[i] {
                return y[i];
            }

            if xout == x[i + 1] {
                return y[i + 1];
            }

            linear_model(x[i], x[i + 1], y[i], y[i + 1], xout)
        })
        .collect()
}

fn linear_model(x1: f64, x2: f64, y1: f64, y2: f64, xout: f64) -> f64 {
    y1 + (y2 - y1) * ((xout - x1) / (x2 - x1))
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rscarbon;
    fn rust_calibrate;
    impl Calibration;
}
