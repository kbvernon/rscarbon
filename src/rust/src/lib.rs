use extendr_api::prelude::*;
use faer::sparse::*;
use rayon::prelude::*;
use statrs::distribution::{Continuous, Normal};

#[extendr]
fn rs_interpolate_linear(x: &[f64], y: &[f64], xout: &[f64]) -> Vec<f64> {

    xout.iter()
        .map(|&xout| {

            let i = x.partition_point(|&y| y >= xout) - 1usize;

            if xout == x[i] { return y[i]; }

            if xout == x[i + 1] { return y[i + 1]; }

            y[i] + (y[i + 1] - y[i]) * ((xout - x[i]) / (x[i + 1] - x[i]))

        })
        .collect()

}

#[extendr]
fn rs_calibrate(
    c14_age: &[f64],
    c14_error: &[f64],
    cal_age: &[f64],
    est_age: &[f64],
    est_error: &[f64],
    precision: f64
) -> ExternalPtr<SparseColMat<usize, f64>> {

    // <i, j> is the <row, column> coordinate in the csc matrix

    let res = (c14_age, c14_error).into_par_iter()
        .enumerate()
        .flat_map(|(i, (&c14_mu, &c14_s))| {
            (est_age, est_error).into_par_iter()
                .enumerate()
                .filter_map(|(j, (&est_mu, &est_s))| {

                    let e = 2i32;

                    let total_error = (c14_s.powi(e) + est_s.powi(e)).sqrt();

                    let gaussian = Normal::new(est_mu, total_error).unwrap();

                    let d = gaussian.pdf(c14_mu);

                    if d < precision { None } else { Some((i, j, d)) }

                })
                .collect::<Vec<(usize, usize, f64)>>()
        })
        .collect::<Vec<(usize, usize, f64)>>();

    let grid = SparseColMat::<usize, f64>::try_new_from_triplets(
        c14_age.len(),
        cal_age.len(),
        &res
    ).unwrap();

    ExternalPtr::new(grid)

}

pub trait SparseSums {
    fn row_sums(&self) -> Vec<f64>;
    fn col_sums(&self) -> Vec<f64>;
}
impl SparseSums for SparseColMatRef<'_, usize, f64> {
    fn row_sums(&self) -> Vec<f64> {
        let row_indices = self.row_indices();
        let col_ptrs = self.col_ptrs();

        let mut rowsums = vec![0_f64; self.nrows()];
        for col in 0..self.ncols() {
            let col_row_indices = &row_indices[col_ptrs[col]..col_ptrs[col + 1]];
            for row in col_row_indices {
                rowsums[*row] += self.get(*row, col).unwrap();
            }
        }

        rowsums
    }

    fn col_sums(&self) -> Vec<f64> {
        let row_indices = self.row_indices();
        let col_ptrs = self.col_ptrs();

        let mut rowsums = vec![0_f64; self.ncols()];
        for col in 0..self.ncols() {
            let col_row_indices = &row_indices[col_ptrs[col]..col_ptrs[col + 1]];
            for row in col_row_indices {
                rowsums[col] += self.get(*row, col).unwrap();
            }
        }

        rowsums
    }
}

#[extendr]
fn rowsums(mat: ExternalPtr<SparseColMat<usize, f64>>) -> Vec<f64> {
    let mat = mat.as_ref();
    mat.as_ref().row_sums()
}

#[cfg(test)]
mod tests {
    use faer::Mat;

    use super::*;

    #[test]
    fn test_sparse_rowsums() {
        let id5: Mat<f64> = Mat::identity(4, 8);
        let sparse_id5 = SparseColMat::<usize, f64>::try_new_from_triplets(
            id5.nrows(),
            id5.ncols(),
            &[(0, 0, 1.), (1, 1, 1.), (2, 2, 1.), (3, 3, 1.)],
        )
        .unwrap();
        assert_eq!(sparse_id5.as_ref().row_sums(), vec![1., 1., 1., 1.]);
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rscarbon;
    fn rs_interpolate_linear;
    fn rs_calibrate;
}
