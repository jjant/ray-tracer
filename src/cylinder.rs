use std::f64::{INFINITY, NEG_INFINITY};

use crate::{misc::EPSILON, ray::Ray, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cylinder {
    pub fn new() -> Self {
        Self {
            minimum: NEG_INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if a.abs() < EPSILON {
            return self.intersect_caps(ray);
        }

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.;

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            vec![]
        } else {
            let t0 = (-b - disc.sqrt()) / (2. * a);
            let t1 = (-b + disc.sqrt()) / (2. * a);

            let y0 = ray.origin.y + t0 * ray.direction.y;
            let y1 = ray.origin.y + t1 * ray.direction.y;
            let mut xs = Vec::with_capacity(2);

            if self.minimum < y0 && y0 < self.maximum {
                xs.push(t0);
            }

            if self.minimum < y1 && y1 < self.maximum {
                xs.push(t1);
            }

            xs.append(&mut self.intersect_caps(ray));
            xs
        }
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);

        if dist < 1. && local_point.y >= self.maximum - EPSILON {
            Tuple::vector(0., 1., 0.)
        } else if dist < 1. && local_point.y <= self.minimum + EPSILON {
            Tuple::vector(0., -1., 0.)
        } else {
            Tuple::vector(local_point.x, 0., local_point.z)
        }
    }

    fn intersect_caps(&self, ray: Ray) -> Vec<f64> {
        let mut xs = Vec::with_capacity(2);

        if !self.closed || ray.direction.y.abs() < EPSILON {
            return xs;
        }

        let t_min = (self.minimum - ray.origin.y) / ray.direction.y;
        let t_max = (self.maximum - ray.origin.y) / ray.direction.y;

        [t_min, t_max]
            .into_iter()
            .filter(|t| check_cap(ray, *t))
            .for_each(|t| xs.push(t));

        xs
    }
}

fn check_cap(ray: Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    let r = 1.;

    x.powi(2) + z.powi(2) <= r
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Make sure this is fine: we don't really want == for f64s,
        // but I don't think we can use approx_equal because we have infinities involved
        self.minimum == other.minimum && self.maximum == other.maximum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{misc::approx_equal, ray::Ray, tuple::Tuple};

    #[test]
    fn a_ray_misses_a_cylinder() {
        let examples = vec![
            (Tuple::point(1., 0., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 0., -5.), Tuple::vector(1., 1., 1.)),
        ];
        let cyl = Cylinder::new();

        for (origin, direction) in examples {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);

            assert!(xs.is_empty());
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let examples = vec![
            (Tuple::point(1., 0., -5.), Tuple::vector(0., 0., 1.), 5., 5.),
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), 4., 6.),
            (
                Tuple::point(0.5, 0., -5.),
                Tuple::vector(0.1, 1., 1.),
                6.80798,
                7.08872,
            ),
        ];

        let cyl = Cylinder::new();
        for (origin, direction, t0, t1) in examples {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), 2);
            assert!(approx_equal(xs[0], t0));
            assert!(approx_equal(xs[1], t1));
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let examples = vec![
            (Tuple::point(1., 0., 0.), Tuple::vector(1., 0., 0.)),
            (Tuple::point(0., 5., -1.), Tuple::vector(0., 0., -1.)),
            (Tuple::point(0., -2., 1.), Tuple::vector(0., 0., 1.)),
            (Tuple::point(-1., 1., 0.), Tuple::vector(-1., 0., 0.)),
        ];

        let cyl = Cylinder::new();
        for (point, expected_normal) in examples {
            let normal = cyl.local_normal_at(point);

            assert_eq!(normal, expected_normal);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(cyl.minimum, NEG_INFINITY);
        assert_eq!(cyl.maximum, INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let examples = vec![
            (Tuple::point(0., 1.5, 0.), Tuple::vector(0.1, 1., 0.), 0),
            (Tuple::point(0., 3., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 1.5, -2.), Tuple::vector(0., 0., 1.), 2),
        ];

        let mut cyl = Cylinder::new();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        for (point, direction, count) in examples {
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), count)
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert!(!cyl.closed);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let mut cyl = Cylinder::new();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        cyl.closed = true;

        let examples = vec![
            (Tuple::point(0., 3., 0.), Tuple::vector(0., -1., 0.), 2),
            (Tuple::point(0., 3., -2.), Tuple::vector(0., -1., 2.), 2),
            (Tuple::point(0., 4., -2.), Tuple::vector(0., -1., 1.), 2),
            (Tuple::point(0., 0., -2.), Tuple::vector(0., 1., 2.), 2),
            (Tuple::point(0., -1., -2.), Tuple::vector(0., 1., 1.), 2),
        ];

        for (point, direction, count) in examples {
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let mut cyl = Cylinder::new();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        cyl.closed = true;

        let examples = vec![
            (Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.)),
            (Tuple::point(0.5, 1., 0.), Tuple::vector(0., -1., 0.)),
            (Tuple::point(0., 1., 0.5), Tuple::vector(0., -1., 0.)),
            (Tuple::point(0., 2., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0.5, 2., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 2., 0.5), Tuple::vector(0., 1., 0.)),
        ];

        for (point, normal) in examples {
            let n = cyl.local_normal_at(point);

            assert_eq!(n, normal)
        }
    }
}
