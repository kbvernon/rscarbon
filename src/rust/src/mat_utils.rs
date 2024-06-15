use faer::sparse::*;

pub trait SparseSums {
    fn col_sums(&self) -> Vec<f64>;
    fn row_sums(&self) -> Vec<f64>;
}

impl SparseSums for SparseColMatRef<'_, usize, f64> {

    fn col_sums(&self) -> Vec<f64> {

        let row_indices = self.row_indices();
        let col_ptrs = self.col_ptrs();

        let mut colsums = vec![0_f64; self.ncols()];

        for col in 0..self.ncols() {

            let s = col_ptrs[col];
            let e = col_ptrs[col + 1];

            let col_row_indices = &row_indices[s..e];

            for row in col_row_indices {
                colsums[col] += self.get(*row, col).unwrap();
            }

        }

        colsums

    }

    fn row_sums(&self) -> Vec<f64> {

        // might consider col_sums(self.transpose()) at some point

        let row_indices = self.row_indices();
        let col_ptrs = self.col_ptrs();

        let mut rowsums = vec![0_f64; self.nrows()];

        for col in 0..self.ncols() {

            let s = col_ptrs[col];
            let e = col_ptrs[col + 1];

            let col_row_indices = &row_indices[s..e];

            for row in col_row_indices {
                rowsums[*row] += self.get(*row, col).unwrap();
            }

        }

        rowsums

    }

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
        assert_eq!(
            sparse_id5.as_ref().row_sums(),
            vec![1., 1., 1., 1.]
        );
        assert_eq!(
            sparse_id5.as_ref().col_sums(),
            vec![1., 1., 1., 1., 0., 0., 0., 0.]
        );
    }
}
