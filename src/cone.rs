use std::f64::{INFINITY, NEG_INFINITY};

use crate::{misc::EPSILON, ray::Ray, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub struct Cone {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cone {
    pub fn new() -> Self {
        Self {
            minimum: NEG_INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64> {
        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2. * ray.origin.x * ray.direction.x - 2. * ray.origin.y * ray.direction.y
            + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        if a.abs() < EPSILON && b.abs() < EPSILON {
            return vec![];
        }

        let mut xs = Vec::with_capacity(4);

        if a.abs() < EPSILON {
            xs.push(-c / (2. * b));
        }

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            return vec![];
        } else {
            let t0 = (-b - disc.sqrt()) / (2. * a);
            let t1 = (-b + disc.sqrt()) / (2. * a);

            let y0 = ray.origin.y + t0 * ray.direction.y;
            let y1 = ray.origin.y + t1 * ray.direction.y;

            if self.minimum < y0 && y0 < self.maximum {
                xs.push(t0);
            }

            if self.minimum < y1 && y1 < self.maximum {
                xs.push(t1);
            }

            xs.append(&mut self.intersect_caps(ray));
        }

        xs
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);
        let y_2 = local_point.y.powi(2);
        let y = if local_point.y > 0. {
            -dist.sqrt()
        } else {
            dist.sqrt()
        };

        if dist < y_2 && local_point.y >= self.maximum - EPSILON {
            Tuple::vector(0., 1., 0.)
        } else if dist < y_2 && local_point.y <= self.minimum + EPSILON {
            Tuple::vector(0., -1., 0.)
        } else {
            Tuple::vector(local_point.x, y, local_point.z)
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
    let y = ray.origin.y + t * ray.direction.y;
    let z = ray.origin.z + t * ray.direction.z;

    x.powi(2) + z.powi(2) <= y.powi(2)
}

impl PartialEq for Cone {
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
    fn intersecting_a_cone_with_a_ray() {
        let examples = vec![
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), 5., 5.),
            (
                Tuple::point(0., 0., -5.),
                Tuple::vector(1., 1., 1.),
                8.66025,
                8.66025,
            ),
            (
                Tuple::point(1., 1., -5.),
                Tuple::vector(-0.5, -1., 1.),
                4.55006,
                49.44994,
            ),
        ];

        let cone = Cone::new();

        for (point, direction, t0, t1) in examples {
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = cone.local_intersect(r);

            assert_eq!(xs.len(), 2);
            assert!(approx_equal(xs[0], t0));
            assert!(approx_equal(xs[1], t1));
        }
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let mut cone = Cone::new();
        cone.minimum = -0.5;
        cone.maximum = 0.5;
        cone.closed = true;

        let examples = vec![
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.), 0),
            (Tuple::point(0., 0., -0.25), Tuple::vector(0., 1., 1.), 2),
            (Tuple::point(0., 0., -0.25), Tuple::vector(0., 1., 0.), 4),
        ];

        for (origin, direction, count) in examples {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = cone.local_intersect(r);

            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let cone = Cone::new();

        // TODO: I believe these normals are incorrect as they're not normalized.
        // They, however, are how they appear in the book.
        let examples = vec![
            (Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 0.)),
            (
                Tuple::point(1., 1., 1.),
                Tuple::vector(1., -(2_f64.sqrt()), 1.),
            ),
            (Tuple::point(-1., -1., 0.), Tuple::vector(-1., 1., 0.)),
        ];

        for (point, normal) in examples {
            let n = cone.local_normal_at(point);

            assert_eq!(n, normal);
        }
    }
}
