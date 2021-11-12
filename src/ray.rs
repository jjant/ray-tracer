#![allow(dead_code)]
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
struct Sphere {
    center: Tuple,
    radius: f64,
}

impl Sphere {
    fn new(center: Tuple, radius: f64) -> Self {
        Self { center, radius }
    }
}

#[derive(Clone, Copy, Debug)]
struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    fn position(self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

// Return the "t" values at which the ray hit the sphere.
fn intersect(ray: Ray, sphere: Sphere) -> Vec<f64> {
    let sphere_to_ray = ray.origin - sphere.center;
    let a = ray.direction.dot(ray.direction);
    let b = 2. * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

    let discriminant = b.powi(2) - 4. * a * c;

    if discriminant < 0. {
        vec![]
    } else {
        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        vec![t1, t2]
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::approx_equal;

    use super::*;

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
        let s = Sphere::new(Tuple::point(0., 0., 0.), 1.);

        let xs = intersect(r, s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0], 4.));
        assert!(approx_equal(xs[1], 6.));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new(Tuple::point(0., 0., 0.), 1.);

        let xs = intersect(r, s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0], 5.));
        assert!(approx_equal(xs[1], 5.));
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let xs = intersect(r, s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new(Tuple::point(0., 0., 0.), 1.);

        let xs = intersect(r, s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0], -1.0));
        assert!(approx_equal(xs[1], 1.0));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new(Tuple::point(0., 0., 0.), 1.);

        let xs = intersect(r, s);

        assert_eq!(xs.len(), 2);
        assert!(approx_equal(xs[0], -6.0));
        assert!(approx_equal(xs[1], -4.0));
    }
}
