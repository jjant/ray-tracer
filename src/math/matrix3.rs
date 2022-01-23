use super::matrix2::Matrix2;
use crate::misc::{self, approx_equal};

const N: usize = 3;
type Row = [f64; N];

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    rows: [Row; N],
}

impl Matrix3 {
    pub fn determinant(&self) -> f64 {
        (0..N)
            .map(|col| {
                let element = self.get(0, col);

                element * self.cofactor(0, col)
            })
            .sum()
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.rows[row][col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.rows[row][col]
    }

    pub fn submatrix(&self, row_to_delete: usize, col_to_delete: usize) -> Matrix2 {
        let mut result = Matrix2::zeroes();

        for row in 0..N {
            for col in 0..N {
                if let Some((offset_row, offset_col)) =
                    misc::cmp_to_offset(row.cmp(&row_to_delete), col.cmp(&col_to_delete))
                {
                    let actual_row = (row as i32 + offset_row) as usize;
                    let actual_col = (col as i32 + offset_col) as usize;

                    *result.get_mut(actual_row, actual_col) = self.get(row, col);
                }
            }
        }
        result
    }

    pub fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }

    fn minor(&self, row_to_delete: usize, col_to_delete: usize) -> f64 {
        self.submatrix(row_to_delete, col_to_delete).determinant()
    }

    fn cofactor(&self, row_to_delete: usize, col_to_delete: usize) -> f64 {
        let row_sign = if row_to_delete % 2 == 0 { 1 } else { -1 };
        let col_sign = if col_to_delete % 2 == 0 { 1 } else { -1 };
        let sign = row_sign * col_sign;

        sign as f64 * self.minor(row_to_delete, col_to_delete)
    }
}

impl PartialEq for Matrix3 {
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
    macro_rules! matrix3 { ($(| $( $x:literal )|* |)*) => { { Matrix3::from_rows([ $([ $( $x as f64, )* ],)* ]) } }; }

    impl Matrix3 {
        pub fn from_rows(rows: [Row; N]) -> Self {
            Self { rows }
        }

        pub fn identity() -> Self {
            let mut zeroes = Self::zeroes();

            (0..N).for_each(|index| {
                *zeroes.get_mut(index, index) = 1.;
            });

            zeroes
        }
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = matrix3![
            | -3| 5 | 0 |
            | 1 | -2| -7|
            | 0 | 1 | 1 |
        ];

        assert!(approx_equal(m.get(0, 0), -3.));
        assert!(approx_equal(m.get(1, 1), -2.));
        assert!(approx_equal(m.get(2, 2), 1.));
    }

    #[test]
    fn identity_works_in_3x3_matrices() {
        let id3 = Matrix3::identity();

        assert_eq!(
            id3,
            Matrix3::from_rows([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = matrix3![
            | 1 | 5 | 0 |
            | -3 | 2 | 7 |
            | 0 | 6 | -3 |
        ];

        assert_eq!(
            a.submatrix(0, 2),
            Matrix2::from_rows([[-3., 2.], [0., 6.],])
        );
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = matrix3![
            | 3 | 5 | 0 |
            | 2 | -1 | -7 |
            | 6 | -1 | 5 |
        ];

        let b = a.submatrix(1, 0);

        assert_eq!(b, Matrix2::from_rows([[5., 0.], [-1., 5.]]));
        assert_eq!(b.determinant(), 25.);
        assert_eq!(a.minor(1, 0), 25.);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = matrix3![
            | 3 | 5 | 0 |
            | 2 | -1 | -7 |
            | 6 | -1 | 5 |
        ];

        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = matrix3![
            | 1 | 2 | 6 |
            | -5 | 8 | -4 |
            | 2 | 6 | 4 |
        ];

        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }
}
