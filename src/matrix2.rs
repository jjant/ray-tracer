use crate::misc::approx_equal;

const N: usize = 2;
type Row = [f64; N];

#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    rows: [Row; N],
}

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }

    pub fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.rows[row][col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.rows[row][col]
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.rows
            .iter()
            .zip(other.rows.iter())
            .all(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .all(|(a, b)| approx_equal(*a, *b))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::approx_equal;

    impl Matrix2 {
        pub fn from_rows(rows: [Row; N]) -> Self {
            Self { rows }
        }

        fn identity() -> Self {
            Self {
                rows: [[1., 0.], [0., 1.]],
            }
        }
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix2::from_rows([[-3., 5.], [1., -2.]]);

        assert!(approx_equal(m.get(0, 0), -3.));
        assert!(approx_equal(m.get(0, 1), 5.));
        assert!(approx_equal(m.get(1, 0), 1.));
        assert!(approx_equal(m.get(1, 1), -2.));
    }

    #[test]
    fn identity_works_in_2x2_matrices() {
        let id2: Matrix2 = Matrix2::identity();

        assert_eq!(id2, Matrix2::from_rows([[1., 0.], [0., 1.]]));
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix2::from_rows([[1., 5.], [-3., 2.]]);

        assert!(approx_equal(a.determinant(), 17.));
    }
}
