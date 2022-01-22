use crate::misc::approx_equal;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn is_point(self) -> bool {
        approx_equal(self.w, 1.0)
    }

    pub fn is_vector(self) -> bool {
        approx_equal(self.w, 0.0)
    }

    pub fn magnitude(self) -> f64 {
        let Self { x, y, z, .. } = self;
        assert!(self.is_vector());

        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn magnitude_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn cross(self, other: Self) -> Self {
        // It's basically a bug if this is not the case.
        // Adding these is probably stupid,
        // I just don't trust myself not to make this error, for now.
        assert!(self.is_vector());
        assert!(other.is_vector());

        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(self, normal: Tuple) -> Self {
        self - normal * 2. * self.dot(normal)
    }

    pub(crate) fn zip_with(&self, other: &Self, f: impl Fn(f64, f64) -> f64) -> Self {
        Self {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
            w: f(self.w, other.w),
        }
    }

    /// component-wise min
    pub(crate) fn min(&self, other: &Self) -> Self {
        self.zip_with(other, f64::min)
    }

    /// component-wise max
    pub(crate) fn max(&self, other: &Self) -> Self {
        self.zip_with(other, f64::max)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        approx_equal(self.x, other.x)
            && approx_equal(self.y, other.y)
            && approx_equal(self.z, other.z)
            && approx_equal(self.w, other.w)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert!(approx_equal(a.x, 4.3));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_is_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert!(approx_equal(a.x, 4.3));
        assert!(approx_equal(a.y, -4.2));
        assert!(approx_equal(a.z, 3.1));
        assert!(approx_equal(a.w, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_point() {
        let p = Tuple::point(4., -4., 3.);

        assert_eq!(p, Tuple::new(4., -4., 3., 1.));
    }

    #[test]
    fn vector_creates_vector() {
        let p = Tuple::vector(4., -4., 3.);

        assert_eq!(p, Tuple::new(4., -4., 3., 0.));
    }

    #[test]
    fn addition_works() {
        let a1 = Tuple::new(3., -2., 5., 1.);
        let a2 = Tuple::new(-2., 3., 1., 0.);

        assert_eq!(a1 + a2, Tuple::new(1., 1., 6., 1.));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3., 2., 1.);
        let p2 = Tuple::point(5., 6., 7.);
        assert_eq!(p1 - p2, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3., 2., 1.);
        let v = Tuple::vector(5., 6., 7.);
        assert_eq!(p - v, Tuple::point(-2., -4., -6.));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3., 2., 1.);
        let v2 = Tuple::vector(5., 6., 7.);
        assert_eq!(v1 - v2, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);

        assert_eq!(-a, Tuple::new(-1., 2., -3., 4.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);

        assert_eq!(a * 0.5, Tuple::new(0.5, -1., 1.5, -2.));
        assert_eq!(0.5 * a, Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn magnitude_works() {
        let v = Tuple::vector(1., 0., 0.);
        assert_eq!(v.magnitude(), 1.);

        let v = Tuple::vector(0., 1., 0.);
        assert_eq!(v.magnitude(), 1.);

        let v = Tuple::vector(0., 0., 1.);
        assert_eq!(v.magnitude(), 1.);

        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());

        let v = Tuple::vector(-1., -2., -3.);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalize_works() {
        let v = Tuple::vector(4., 0., 0.);
        assert_eq!(v.normalize(), Tuple::vector(1., 0., 0.));

        let v = Tuple::vector(1., 2., 3.);
        // Tuple::vector(1/√14, 2/√14, 3/√14)
        assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
        let v = Tuple::vector(1., 2., 3.);
        let norm = v.normalize();
        assert_eq!(norm.magnitude(), 1.);
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);

        assert_eq!(a.dot(b), 20.);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);

        assert_eq!(a.cross(b), Tuple::vector(-1., 2., -1.));
        assert_eq!(b.cross(a), Tuple::vector(1., -2., 1.));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = Tuple::vector(1., -1., 0.);
        let n = Tuple::vector(0., 1., 0.);
        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1., 1., 0.))
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Tuple::vector(0., -1., 0.);
        let n = Tuple::vector(2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.);
        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1., 0., 0.))
    }
}
