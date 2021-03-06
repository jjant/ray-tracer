use crate::math::matrix4::Matrix4;
use crate::math::tuple::Tuple;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::approx_equal;
    use crate::shape::{Object, SimpleObject};

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
        let s = Object::sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, 4.));
        assert!(approx_equal(xs[1].t, 6.));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Object::sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, 5.));
        assert!(approx_equal(xs[1].t, 5.));
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Object::sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Object::sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, -1.0));
        assert!(approx_equal(xs[1].t, 1.0));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Object::sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, -6.0));
        assert!(approx_equal(xs[1].t, -4.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let object = Object::sphere();
        let s = SimpleObject::from_object(&object).unwrap();
        let xs = object.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
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
        let mut object = Object::sphere();
        object.transform = Matrix4::scaling(2., 2., 2.);
        let s = SimpleObject::from_object(&object).unwrap();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0].t, 3.));
        assert!(approx_equal(xs[1].t, 7.));
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut object = Object::sphere();
        object.transform = Matrix4::translation(5., 0., 0.);
        let s = SimpleObject::from_object(&object).unwrap();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }
}
