use std::ops::Mul;

use super::matrix3::Matrix3;
use super::tuple::Tuple;
use crate::misc::{self, approx_equal};

const N: usize = 4;
type Row = [f64; N];

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    rows: [Row; N],
}

impl Matrix4 {
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

    pub fn transpose(&self) -> Self {
        let mut result = Self::zeroes();

        for row in 0..N {
            for col in 0..N {
                *result.get_mut(col, row) = self.get(row, col);
            }
        }

        result
    }

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

    pub fn submatrix(&self, row_to_delete: usize, col_to_delete: usize) -> Matrix3 {
        let mut result = Matrix3::zeroes();

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

    fn zeroes() -> Self {
        Self { rows: [[0.; N]; N] }
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();

        if approx_equal(det, 0.) {
            None
        } else {
            let mut result = Matrix4::zeroes();

            for row in 0..N {
                for col in 0..N {
                    let cofactor = self.cofactor(row, col);

                    *result.get_mut(col, row) = cofactor / det;
                }
            }

            Some(result)
        }
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

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self::from_rows([
            [1., 0., 0., x],
            [0., 1., 0., y],
            [0., 0., 1., z],
            [0., 0., 0., 1.],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self::from_rows([
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_x(angle_radians: f64) -> Self {
        let r = angle_radians;
        Self::from_rows([
            [1., 0., 0., 0.],
            [0., r.cos(), -r.sin(), 0.],
            [0., r.sin(), r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_y(angle_radians: f64) -> Self {
        let r = angle_radians;
        Self::from_rows([
            [r.cos(), 0., r.sin(), 0.],
            [0., 1., 0., 0.],
            [-r.sin(), 0., r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_z(angle_radians: f64) -> Self {
        let r = angle_radians;

        Self::from_rows([
            [r.cos(), -r.sin(), 0., 0.],
            [r.sin(), r.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    #[allow(dead_code)]
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::from_rows([
            [1., xy, xz, 0.],
            [yx, 1., yz, 0.],
            [zx, zy, 1., 0.],
            [0., 0., 0., 1.],
        ])
    }
}

impl Mul for Matrix4 {
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

impl Mul<f64> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let rows = self.rows;
        let new_rows = [
            [
                rows[0][0] * rhs,
                rows[0][1] * rhs,
                rows[0][2] * rhs,
                rows[0][3] * rhs,
            ],
            [
                rows[1][0] * rhs,
                rows[1][1] * rhs,
                rows[1][2] * rhs,
                rows[1][3] * rhs,
            ],
            [
                rows[2][0] * rhs,
                rows[2][1] * rhs,
                rows[2][2] * rhs,
                rows[2][3] * rhs,
            ],
            [
                rows[3][0] * rhs,
                rows[3][1] * rhs,
                rows[3][2] * rhs,
                rows[3][3] * rhs,
            ],
        ];

        Self { rows: new_rows }
    }
}
impl Mul<Tuple> for Matrix4 {
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

fn row_to_tuple(row: Row) -> Tuple {
    Tuple::new(row[0], row[1], row[2], row[3])
}

impl PartialEq for Matrix4 {
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
    use std::f64::consts::PI;
    macro_rules! matrix4 { ($(| $( $x:literal )|* |)*) => { { Matrix4::from_rows([ $([ $( $x as f64, )* ],)* ]) } }; }

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix4::from_rows([
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
    fn matrix_equality_with_identical_matrices() {
        let a = matrix4![
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ];

        let b = matrix4![
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ];

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = matrix4![
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ];
        let b = matrix4![
            | 2 | 3 | 4 | 5 |
            | 6 | 7 | 8 | 9 |
            | 8 | 7 | 6 | 5 |
            | 4 | 3 | 2 | 1 |
        ];

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let b = Matrix4::from_rows([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);

        let c = Matrix4::from_rows([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);

        assert_eq!(a * b, c);
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let a = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);
        let b = Tuple::new(1., 2., 3., 1.);

        assert_eq!(a * b, Tuple::new(18., 24., 33., 1.));
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = Tuple::new(1., 2., 3., 4.);

        assert_eq!(Matrix4::identity() * a, a);
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = matrix4![
            | 0 | 1 | 2 | 4 |
            | 1 | 2 | 4 | 8 |
            | 2 | 4 | 8 | 16 |
            | 4 | 8 | 16 | 32 |
        ];

        assert_eq!(a * Matrix4::identity(), a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = matrix4![
            | 0 | 9 | 3 | 0 |
            | 9 | 8 | 0 | 8 |
            | 1 | 8 | 5 | 3 |
            | 0 | 0 | 5 | 8 |
        ];

        let expected_transpose = matrix4![
            | 0 | 9 | 1 | 0 |
            | 9 | 8 | 8 | 0 |
            | 3 | 0 | 5 | 5 |
            | 0 | 8 | 3 | 8 |
        ];

        assert_eq!(a.transpose(), expected_transpose);
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let id4 = Matrix4::identity();

        assert_eq!(id4, id4.transpose())
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = matrix4![
            | -6 | 1 | 1 | 6 |
            | -8 | 5 | 8 | 6 |
            | -1 | 0 | 8 | 2 |
            | -7 | 1 | -1 | 1 |
        ];

        let submatrix = Matrix3::from_rows([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);

        assert_eq!(a.submatrix(2, 1), submatrix);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = matrix4![
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

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = matrix4![
            | 6 | 4 | 4 | 4 |
            | 5 | 5 | 7 | 6 |
            | 4 | -9 | 3 | -7 |
            | 9 | 1 | 7 | -6 |
        ];

        assert!(approx_equal(a.determinant(), -2120.));
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = matrix4![
            | -4 | 2 | -2 | -3 |
            | 9 | 6 | 2 | 6 |
            | 0 | -5 | 1 | -5 |
            | 0 | 0 | 0 | 0 |
        ];

        assert!(approx_equal(a.determinant(), 0.));
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = matrix4![
            | -5 | 2 | 6 | -8 |
            | 1 | -5 | 1 | 8 |
            | 7 | 7 | -6 | -7 |
            | 1 | -3 | 7 | 4 |
        ];
        let b = a.inverse().unwrap();

        assert!(approx_equal(a.determinant(), 532.));
        assert!(approx_equal(a.cofactor(2, 3), -160.));
        assert!(approx_equal(b.get(3, 2), -160. / 532.));
        assert!(approx_equal(a.cofactor(3, 2), 105.));
        assert!(approx_equal(b.get(2, 3), 105. / 532.));
        assert_eq!(
            b,
            matrix4![
                | 0.21805 | 0.45113 | 0.24060 | -0.04511 |
                | -0.80827 | -1.45677 | -0.44361 | 0.52068 |
                | -0.07895 | -0.22368 | -0.05263 | 0.19737 |
                | -0.52256 | -0.81391 | -0.30075 | 0.30639 |
            ]
        );
    }
    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = matrix4![
            | 8 | -5 | 9 | 2 |
            | 7 | 5 | 6 | 1 |
            | -6 | 0 | 9 | 6 |
            | -3 | 0 | -9 | -4 |
        ];

        let expected_inverse = matrix4![
            | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
            | -0.07692 | 0.12308 | 0.02564 | 0.03077 |
            | 0.35897 | 0.35897 | 0.43590 | 0.92308 |
            | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
        ];

        assert_eq!(a.inverse().unwrap(), expected_inverse);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a = matrix4![
            | 9 | 3 | 0 | 9 |
            | -5 | -2 | -6 | -3 |
            | -4 | 9 | 6 | 4 |
            | -7 | 6 | 6 | 2 |
        ];

        let expected_inverse = matrix4![
            | -0.04074 | -0.07778 | 0.14444 | -0.22222 |
            | -0.07778 | 0.03333 | 0.36667 | -0.33333 |
            | -0.02901 | -0.14630 | -0.10926 | 0.12963 |
            | 0.17778 | 0.06667 | -0.26667 | 0.33333 |
        ];

        assert_eq!(a.inverse().unwrap(), expected_inverse);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = matrix4![
            | 3 | -9 | 7 | 3 |
            | 3 | -8 | 2 | -9 |
            | -4 | 4 | 4 | 1 |
            | -6 | 5 | -1 | 1 |
        ];
        let b = matrix4![
            | 8 | 2 | 2 | 2 |
            | 3 | -1 | 7 | 0 |
            | 7 | 0 | 5 | 4 |
            | 6 | -2 | 0 | 5 |
        ];

        let c = a * b;
        assert_eq!(c * b.inverse().unwrap(), a);
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix4::translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(transform * p, Tuple::point(2., 1., 7.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix4::translation(5., -3., 2.);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(inv * p, Tuple::point(-8., 7., 3.));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix4::translation(5., -3., 2.);
        let v = Tuple::vector(-3., 4., 5.);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix4::scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);

        assert_eq!(transform * p, Tuple::point(-8., 18., 32.));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix4::scaling(2., 3., 4.);
        let v = Tuple::vector(-4., 6., 8.);

        assert_eq!(transform * v, Tuple::vector(-8., 18., 32.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix4::scaling(2., 3., 4.);
        let inv = transform.inverse().unwrap();
        let v = Tuple::vector(-4., 6., 8.);

        assert_eq!(inv * v, Tuple::vector(-2., 2., 2.));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix4::scaling(-1., 1., 1.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(-2., 3., 4.));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix4::rotation_x(PI / 4.);
        let full_quarter = Matrix4::rotation_x(PI / 2.);

        assert_eq!(
            half_quarter * p,
            Tuple::point(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
        );
        assert_eq!(full_quarter * p, Tuple::point(0., 0., 1.));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix4::rotation_x(PI / 4.);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            inv * p,
            Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0., 0., 1.);
        let half_quarter = Matrix4::rotation_y(PI / 4.);
        let full_quarter = Matrix4::rotation_y(PI / 2.);

        assert_eq!(
            half_quarter * p,
            Tuple::point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.)
        );
        assert_eq!(full_quarter * p, Tuple::point(1., 0., 0.));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix4::rotation_z(PI / 4.);
        let full_quarter = Matrix4::rotation_z(PI / 2.);

        assert_eq!(
            half_quarter * p,
            Tuple::point(-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.)
        );
        assert_eq!(full_quarter * p, Tuple::point(-1., 0., 0.));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix4::shearing(1., 0., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(5., 3., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix4::shearing(0., 1., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(6., 3., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix4::shearing(0., 0., 1., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(2., 5., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix4::shearing(0., 0., 0., 1., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(2., 7., 4.));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix4::shearing(0., 0., 0., 0., 1., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(2., 3., 6.));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix4::shearing(0., 0., 0., 0., 0., 1.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(2., 3., 7.));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1., 0., 1.);
        let a = Matrix4::rotation_x(PI / 2.);
        let b = Matrix4::scaling(5., 5., 5.);
        let c = Matrix4::translation(10., 5., 7.);

        // Apply rotation first
        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1., -1., 0.));
        // Then apply scaling
        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5., -5., 0.));
        // Then apply translation
        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15., 0., 7.));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1., 0., 1.);
        let a = Matrix4::rotation_x(PI / 2.);
        let b = Matrix4::scaling(5., 5., 5.);
        let c = Matrix4::translation(10., 5., 7.);
        let t = c * b * a;

        assert_eq!(t * p, Tuple::point(15., 0., 7.));
    }
}
