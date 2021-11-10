use crate::{misc::approx_equal, tuple::Tuple};
use std::ops::Mul;

type Row<const N: usize> = [f64; N];

#[derive(Debug)]
pub struct Matrix<const N: usize> {
    // rows: [f64; N * N],
    // ^
    // |
    // This doesn't work right now
    rows: [Row<N>; N],
}

impl<const N: usize> Matrix<N> {
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

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.rows[row][col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.rows[row][col]
    }

    fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Use approx_equal for each element somehow
        self.rows == other.rows
    }
}

impl<const N: usize> Mul for Matrix<N> {
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
}
