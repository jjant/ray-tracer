#![allow(dead_code)]
use crate::matrix4::Matrix4;
use crate::sphere::Sphere;
use crate::{misc::approx_equal, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

impl Intersection {
    fn new(t: f64, object: Sphere) -> Self {
        Self { t, object }
    }

    // Returns the closest intersection, that is
    // the one with the smallest non-negative t value.
    pub fn hit(intersections: &[Self]) -> Option<Self> {
        intersections
            .iter()
            .filter(|i| (**i).t >= 0.)
            .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap())
            .copied()
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.t, other.t) && self.object == other.object
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(self, matrix: Matrix4) -> Self {
        Self {
            origin: matrix * self.origin,
            direction: matrix * self.direction,
        }
    }

    /// The maths assume the sphere is located in the origin,
    /// and it handles the general case by "unmoving" the ray with the opposite transform.
    pub fn intersect(self, sphere: Sphere) -> Vec<Intersection> {
        let ray2 = self.transform(sphere.transform.inverse().unwrap());

        let sphere_to_ray = ray2.origin - Tuple::point(0., 0., 0.);
        let a = ray2.direction.magnitude_squared();
        let b = 2. * ray2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.magnitude_squared() - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);

            vec![Intersection::new(t1, sphere), Intersection::new(t2, sphere)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::approx_equal;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);

        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2., 3., 4.), Tuple::vector(1., 0., 0.));

        assert_eq!(r.position(0.), Tuple::point(2., 3., 4.));
        assert_eq!(r.position(1.), Tuple::point(3., 3., 4.));
        assert_eq!(r.position(-1.), Tuple::point(1., 3., 4.));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3., 4.));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, 4.));
        assert!(approx_equal(xs[1].t, 6.));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, 5.));
        assert!(approx_equal(xs[1].t, 5.));
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = r.intersect(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, -1.0));
        assert!(approx_equal(xs[1].t, 1.0));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, -6.0));
        assert!(approx_equal(xs[1].t, -4.0));
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s);

        assert!(approx_equal(i.t, 3.5));
        assert_eq!(i.object, s);
    }

    // TODO: check if this test is actually needed
    //
    // Scenario: Aggregating intersections
    // Given s = sphere()
    // And i1 = intersection(1, s)
    // And i2 = intersection(2, s)
    // When xs = intersections(i1, i2)
    // Then xs.count = 2
    // And xs[0].t = 1
    // And xs[1].t = 2

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(2., s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1., s);
        let i2 = Intersection::new(1., s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2., s);
        let i2 = Intersection::new(-1., s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5., s);
        let i2 = Intersection::new(7., s);
        let i3 = Intersection::new(-3., s);
        let i4 = Intersection::new(2., s);
        let xs = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(i4));
    }
    #[test]
    fn translating_a_ray_() {
        let r = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let m = Matrix4::translation(3., 4., 5.);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(4., 6., 8.));
        assert_eq!(r2.direction, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn scaling_a_ray_() {
        let r = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let m = Matrix4::scaling(2., 3., 4.);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(2., 6., 12.));
        assert_eq!(r2.direction, Tuple::vector(0., 3., 0.));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::new();

        s.set_transform(Matrix4::scaling(2., 2., 2.));

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, 3.));
        assert!(approx_equal(xs[1].t, 7.));
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::new();

        s.set_transform(Matrix4::translation(5., 0., 0.));

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 0);
    }
}
