use crate::{misc::approx_equal, tuple::Tuple};
use std::compile_error;
use std::{cmp::Ordering, ops::Mul};

macro_rules! matrix { ($(| $( $x:literal )|* |)*) => { { [ $([ $( $x as f64, )* ],)* ] } }; }

macro_rules! matrix_ { ($(| $( $x:literal )|* |)*) => { { Matrix::from_rows([ $([ $( $x as f64, )* ],)* ]) } }; }

type Row<const N: usize> = [f64; N];

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize>
where
    [(); N]: Sized,
{
    // rows: [f64; N * N],
    // ^
    // |
    // This doesn't work right now
    rows: [Row<N>; N],
}

fn cmp_to_offset(ordering_row: Ordering, ordering_col: Ordering) -> Option<(i32, i32)> {
    match (ordering_row, ordering_col) {
        (Ordering::Equal, _) => None,
        (_, Ordering::Equal) => None,
        (Ordering::Greater, Ordering::Greater) => Some((-1, -1)),
        (Ordering::Less, Ordering::Greater) => Some((0, -1)),
        (Ordering::Greater, Ordering::Less) => Some((-1, 0)),
        (Ordering::Less, Ordering::Less) => Some((0, 0)),
    }
}

impl<const N: usize> Matrix<N>
where
    [(); N]: Sized,
{
    pub fn from_rows(rows: [Row<N>; N]) -> Self {
        Self { rows: rows }
    }

    pub fn identity() -> Self {
        let mut zeroes = Self::zeroes();

        (0..N).for_each(|index| {
            *zeroes.get_mut(index, index) = 1.;
        });

        zeroes
    }

    pub fn transpose(&self) -> Self {
        let mut result = Self::zeroes();

        for row in 0..N {
            for col in 0..N {
                *result.get_mut(col, row) = self.get(row, col);
            }
        }

        result
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.rows[row][col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.rows[row][col]
    }

    fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }

    fn determinant(&self) -> f64 {
        match N {
            0 => unreachable!(),
            1 => panic!("Instantiated matrix with size 1 (it's probably a bug by me)"),
            2 => self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0),
            _ => (0..N)
                .map(|col| {
                    let element = self.get(0, col);

                    element * self.cofactor(0, col)
                })
                .sum(),
        }
    }
}

impl<const N: usize> Matrix<N>
where
    [(); N - 1]: Sized,
{
    pub fn submatrix(&self, row_to_delete: usize, col_to_delete: usize) -> Matrix<{ N - 1 }> {
        let mut result = Matrix::zeroes();

        for row in 0..N {
            for col in 0..N {
                if let Some((offset_row, offset_col)) =
                    cmp_to_offset(row.cmp(&row_to_delete), col.cmp(&col_to_delete))
                {
                    let actual_row = (row as i32 + offset_row) as usize;
                    let actual_col = (col as i32 + offset_col) as usize;

                    *result.get_mut(actual_row, actual_col) = self.get(row, col);
                }
            }
        }
        result
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

impl<const N: usize> PartialEq for Matrix<N>
where
    [(); N - 1]: Sized,
{
    fn eq(&self, other: &Self) -> bool {
        // TODO: Use approx_equal for each element somehow
        self.rows == other.rows
    }
}

impl<const N: usize> Mul for Matrix<N>
where
    [(); N - 1]: Sized,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::from_rows([[0.; N]; N]);

        for row in 0..N {
            for col in 0..N {
                for k in 0..N {
                    *result.get_mut(row, col) += self.get(row, k) * rhs.get(k, col);
                }
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Self::Output {
        Tuple::new(
            row_to_tuple(self.rows[0]).dot(tuple),
            row_to_tuple(self.rows[1]).dot(tuple),
            row_to_tuple(self.rows[2]).dot(tuple),
            row_to_tuple(self.rows[3]).dot(tuple),
        )
    }
}

fn row_to_tuple(row: Row<4>) -> Tuple {
    Tuple::new(row[0], row[1], row[2], row[3])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m: Matrix<4> = Matrix::from_rows([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(approx_equal(m.get(0, 0), 1.));
        assert!(approx_equal(m.get(0, 3), 4.));
        assert!(approx_equal(m.get(1, 0), 5.5));
        assert!(approx_equal(m.get(1, 2), 7.5));
        assert!(approx_equal(m.get(2, 2), 11.));
        assert!(approx_equal(m.get(3, 0), 13.5));
        assert!(approx_equal(m.get(3, 2), 15.5));
    }
    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m: Matrix<2> = Matrix::from_rows([[-3., 5.], [1., -2.]]);

        assert!(approx_equal(m.get(0, 0), -3.));
        assert!(approx_equal(m.get(0, 1), 5.));
        assert!(approx_equal(m.get(1, 0), 1.));
        assert!(approx_equal(m.get(1, 1), -2.));
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m: Matrix<3> = Matrix::from_rows([[-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]]);

        assert!(approx_equal(m.get(0, 0), -3.));
        assert!(approx_equal(m.get(1, 1), -2.));
        assert!(approx_equal(m.get(2, 2), 1.));
    }

    // TODO: Translate these tests
    //
    // Scenario: Matrix equality with identical matrices
    // Given the following matrix A:
    // | 1 | 2 | 3 | 4 |
    // | 5 | 6 | 7 | 8 |
    // | 9 | 8 | 7 | 6 |
    // | 5 | 4 | 3 | 2 |
    // And the following matrix B:
    // | 1 | 2 | 3 | 4 |
    // | 5 | 6 | 7 | 8 |
    // | 9 | 8 | 7 | 6 |
    // | 5 | 4 | 3 | 2 |
    // Then A = B
    // Scenario: Matrix equality with different matrices
    // Given the following matrix A:
    // | 1 | 2 | 3 | 4 |
    // | 5 | 6 | 7 | 8 |
    // | 9 | 8 | 7 | 6 |
    // | 5 | 4 | 3 | 2 |
    // report erratum • discuss
    // Creating a Matrix • 27
    // And the following matrix B:
    // | 2 | 3 | 4 | 5 |
    // | 6 | 7 | 8 | 9 |
    // | 8 | 7 | 6 | 5 |
    // | 4 | 3 | 2 | 1 |
    // Then A != B

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::from_rows([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let b = Matrix::from_rows([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);

        let c = Matrix::from_rows([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);

        assert_eq!(a * b, c);
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let a: Matrix<4> = Matrix::from_rows([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);
        let b = Tuple::new(1., 2., 3., 1.);

        assert_eq!(a * b, Tuple::new(18., 24., 33., 1.));
    }

    #[test]
    fn identity_works_in_2x2_matrices() {
        let id2: Matrix<2> = Matrix::identity();

        assert_eq!(id2, Matrix::from_rows([[1., 0.], [0., 1.]]));
    }

    #[test]
    fn identity_works_in_3x3_matrices() {
        let id3: Matrix<3> = Matrix::identity();

        assert_eq!(
            id3,
            Matrix::from_rows([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );
    }
    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix::from_rows(matrix![
            | 0 | 1 | 2 | 4 |
            | 1 | 2 | 4 | 8 |
            | 2 | 4 | 8 | 16 |
            | 4 | 8 | 16 | 32 |
        ]);

        assert_eq!(a * Matrix::identity(), a);
    }
    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = Tuple::new(1., 2., 3., 4.);

        assert_eq!(Matrix::identity() * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::from_rows(matrix![
            | 0 | 9 | 3 | 0 |
            | 9 | 8 | 0 | 8 |
            | 1 | 8 | 5 | 3 |
            | 0 | 0 | 5 | 8 |
        ]);

        let expected_transpose = Matrix::from_rows(matrix![
            | 0 | 9 | 1 | 0 |
            | 9 | 8 | 8 | 0 |
            | 3 | 0 | 5 | 5 |
            | 0 | 8 | 3 | 8 |
        ]);

        assert_eq!(a.transpose(), expected_transpose);
    }
    #[test]
    fn transposing_the_identity_matrix() {
        let id4 = Matrix::<4>::identity();

        assert_eq!(id4, id4.transpose())
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix::from_rows(matrix![
            | 1 | 5 |
            | -3 | 2 |
        ]);

        assert!(approx_equal(a.determinant(), 17.));
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix::from_rows(matrix![
            | 1 | 5 | 0 |
            | -3 | 2 | 7 |
            | 0 | 6 | -3 |
        ]);

        assert_eq!(
            a.submatrix(0, 2),
            Matrix::from_rows(matrix![
                | -3 | 2 |
                | 0 | 6 |
            ])
        );
    }
    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = matrix_![
            | -6 | 1 | 1 | 6 |
            | -8 | 5 | 8 | 6 |
            | -1 | 0 | 8 | 2 |
            | -7 | 1 | -1 | 1 |
        ];

        let submatrix = matrix_![
            | -6 | 1 | 6 |
            | -8 | 8 | 6 |
            | -7 | -1 | 1 |
        ];

        assert_eq!(a.submatrix(2, 1), submatrix);
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = matrix_![
            | 3 | 5 | 0 |
            | 2 | -1 | -7 |
            | 6 | -1 | 5 |
        ];

        let b = a.submatrix(1, 0);

        assert_eq!(
            b,
            matrix_![
                | 5 | 0 |
                | -1| 5 |
            ]
        );
        assert_eq!(b.determinant(), 25.);
        assert_eq!(a.minor(1, 0), 25.);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = matrix_![
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
        let a = matrix_![
            | 1 | 2 | 6 |
            | -5 | 8 | -4 |
            | 2 | 6 | 4 |
        ];

        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }
    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = matrix_![
            | -2 | -8 | 3 | 5 |
            | -3 | 1 | 7 | 3 |
            | 1 | 2 | -9 | 6 |
            | -6 | 7 | 7 | -9 |
        ];

        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }
}
