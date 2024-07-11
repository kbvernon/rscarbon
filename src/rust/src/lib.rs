use extendr_api::prelude::*;

mod c14date;
mod caldate;
mod calibration;
// mod dpmm;

use c14date::{C14Date, C14List};
use caldate::CalGrid;
use calibration::Calibration;

#[extendr]
fn rust_make_c14_vctr(estimate: Vec<i32>, error: Vec<i32>) -> List {
    let dates: Vec<C14Date> = estimate.iter()
        .zip(error.iter())
        .map(|(&u, &v)| C14Date::new(u, v))
        .collect();

    C14List::new(dates).into_list()
}

#[extendr]
fn rust_calibrate(
    x: List, 
    calibration: Calibration, 
    precision: f64, 
    sum_to_one: bool
) -> List {
    let c14list: C14List = x.into();
    let calgrid: CalGrid = c14list.calibrate(
        &calibration, 
        precision, 
        sum_to_one
    );
    
    calgrid.into_list(calibration.name, sum_to_one)
}

#[extendr]
fn rust_interpolate_calibration(x: Robj) -> Robj {
    let mut calibration: Calibration = x.into();
    calibration.interpolate().into()
}

#[extendr]
fn rust_mode(x: List) -> Vec<i32> { 
    let calgrid: CalGrid = x.into();
    calgrid.mode()
 }

/* 
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

        let iterator = cal_date.age.iter().zip(cal_date.density.iter());

        for (&age, &density) in iterator {

            let j: usize = (start - age).try_into().unwrap();

            matrix[[i, j]] = density;

        }

    }

    matrix

}
*/    

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rscarbon;
    fn rust_make_c14_vctr;
    fn rust_calibrate;
    fn rust_interpolate_calibration;
    // use dpmm;
}

