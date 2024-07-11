use extendr_api::prelude::*;

/// Calibration
/// age: Vec<i32> the calendar age
/// estimate: Vec<f64> the calibration age estimate
/// error: Vec<f64> the calibration age error

#[derive(Debug)]
pub struct Calibration {
    pub(crate) name: &'static str,
    pub(crate) age: Vec<i32>,
    pub(crate) estimate: Vec<f64>,
    pub(crate) error: Vec<f64>
}

impl Calibration {
    pub(crate) fn new(
        name: &'static str, 
        age: Vec<i32>, 
        estimate: Vec<f64>, 
        error: Vec<f64>
    ) -> Self { Calibration {name, age, estimate, error} }

    pub(crate) fn interpolate(&mut self) {
        let start: i32 = *self.age.iter().max().unwrap();
        let end: i32 = *self.age.iter().min().unwrap();

        let xout: Vec<i32> = (end..=start).rev().collect();

        (self.estimate, self.error) = xout.into_iter()
            .map(|u: i32| {
                let i: usize = self.age.partition_point(|&v| v >= u) - 1usize;

                if u == self.age[i] { return (self.estimate[i], self.error[i]) }
                if u == self.age[i+1] { return (self.estimate[i+1], self.error[i+1]) }

                let x1: f64 = f64::from(self.age[i]);
                let x2: f64 = f64::from(self.age[i+1]);

                let e_estimate: f64 = linear(
                    x1,
                    x2,
                    self.estimate[i],
                    self.estimate[i+1],
                    f64::from(u)
                );

                let e_error: f64 = linear(
                    x1,
                    x2,
                    self.error[i],
                    self.error[i+1],
                    f64::from(u)
                );

                (e_estimate, e_error)

            })
            .unzip();

        self.age = (end..=start).rev().collect()
    }

}

fn linear(x1: f64, x2: f64, y1: f64, y2: f64, u: f64) -> f64 {
    y1 + (y2 - y1) * ((u - x1) / (x2 - x1))
}

impl From<Robj> for Calibration {
    fn from(x: Robj) -> Self {
        let name: &str = x.get_attrib("curve").unwrap().try_into().unwrap();
        let age: Vec<i32> = x.dollar("age").unwrap().try_into().unwrap();
        let estimate: Vec<f64> = x.dollar("estimate").unwrap().try_into().unwrap();
        let error: Vec<f64> = x.dollar("error").unwrap().try_into().unwrap();

        Calibration::new(name, age, estimate, error)
    }
}

impl From<Calibration> for Robj {
    fn from(x: Calibration) -> Self {
        let mut tbl = list!(
            age=x.age,
            estimate=x.estimate,
            error=x.error
        );

        tbl
            .set_class(["Calibration", "tbl_df", "tbl"])
            .unwrap()
            .set_attrib("curve", x.name)
            .unwrap();
        
        tbl.into_robj()
    }
}