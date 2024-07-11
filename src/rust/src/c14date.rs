use crate::caldate::{CalDate, CalGrid};
use crate::calibration::Calibration;
use extendr_api::prelude::*;
use rayon::prelude::*;
use statrs::distribution::{Continuous, Normal};

#[derive(Debug)]
pub struct C14Date {
    pub(crate) age: i32,
    pub(crate) error: i32
}

#[derive(Debug)]
pub struct C14List {
    pub(crate) dates: Vec<C14Date>
}

// have extendr convert to external pointer
#[extendr]
impl C14Date {}

impl C14Date {
    pub(crate) fn new(age: i32, error: i32) -> Self {
        C14Date {age, error}
    }

    pub(crate) fn calibrate(
        self,
        calibration: &Calibration,
        precision: f64,
        sum_to_one: bool
    ) -> CalDate {    
        let c14_mu = f64::from(self.age);
        let c14_sd2 = f64::from(self.error).powi(2i32);

        let cal_age: &Vec<i32> = &calibration.age;
        let cal_mu_vec: &Vec<f64> = &calibration.estimate;
        let cal_sd_vec: &Vec<f64> = &calibration.error;
    
        let (year, density): (Vec<i32>, Vec<f64>) = cal_mu_vec.iter()
            .zip(cal_sd_vec.iter())
            .enumerate()
            .filter_map(|(i, (&u, &v))| {    
                let cal_mu = f64::from(u);
                let cal_sd2 = f64::from(v).powi(2i32);
    
                let total_error = (c14_sd2 + cal_sd2).sqrt();
    
                let d = dnorm(c14_mu, cal_mu, total_error);
    
                if d < precision { None } else { Some ((cal_age[i], d)) }    
            })
            .unzip();

        let start: i32 = *year.iter().max().unwrap();
        let end: i32 = *year.iter().min().unwrap();

        let mut new_date = CalDate::new(start, end, density);

        if sum_to_one { new_date.normalize() };

        new_date    
    }

}

fn dnorm(x: f64, mean: f64, sd: f64) -> f64 {
    let gaussian = Normal::new(mean, sd).unwrap();
    gaussian.pdf(x)
}

impl C14List {
    pub(crate) fn new(dates: Vec<C14Date>) -> Self {
        C14List {dates}
    }

    pub(crate) fn calibrate(
        self,
        calibration: &Calibration,
        precision: f64,
        sum_to_one: bool
    ) -> CalGrid {
        let grid: CalGrid = self.dates
            .into_par_iter()
            .map(|v: C14Date| v.calibrate(calibration, precision, sum_to_one))
            .collect::<Vec<CalDate>>()
            .into();
        
        grid
    }

    pub(crate) fn into_list(self) -> List {
        let ages: Vec<i32> = self.dates
            .iter()
            .map(|v| v.age)
            .collect();
        let start: i32 = *ages.iter().max().unwrap();
        let end: i32 = *ages.iter().min().unwrap();
        let w = vec![start, end];

        let current_window = Integers::from_values(w);
        let mut list = List::from_values(self.dates);

        list.set_class(["C14List", "vctrs_vctr", "list"])
            .unwrap()
            //.set_attrib("f14c", f14c)
            //.unwrap()
            .set_attrib("window", current_window)
            .unwrap();

        list
    }
}

impl From<List> for C14List {
    fn from(x: List) -> Self {
        let dates: Vec<C14Date> = x.values()
            .map(|v|{
                let date: ExternalPtr<C14Date> = v.try_from().unwrap();
                *date
            })
            .collect();

        C14List::new(dates)
    }
}