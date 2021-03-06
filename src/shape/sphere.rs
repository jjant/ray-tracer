use crate::math::tuple::Tuple;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn local_intersect(local_ray: Ray) -> Vec<f64> {
        let sphere_to_ray = local_ray.origin - Tuple::point(0., 0., 0.);
        let a = local_ray.direction.magnitude_squared();
        let b = 2. * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.magnitude_squared() - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);

            vec![t1, t2]
        }
    }

    pub fn local_normal_at(local_point: Tuple) -> Tuple {
        // Warning: do not remove this (consider the w!)
        local_point - Tuple::point(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        intersection::Intersection,
        shape::{Object, SimpleObject},
    };

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let object = Object::sphere();
        let s = SimpleObject::from_object(&object).unwrap();
        let i = Intersection::new_(0., s);
        let n = s.normal_at(i, Tuple::point(1., 0., 0.));
        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let object = Object::sphere();
        let s = SimpleObject::from_object(&object).unwrap();
        let i = Intersection::new_(0., s);
        let n = s.normal_at(i, Tuple::point(0., 1., 0.));
        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let object = Object::sphere();
        let s = SimpleObject::from_object(&object).unwrap();
        let i = Intersection::new_(0., s);
        let n = s.normal_at(i, Tuple::point(0., 0., 1.));
        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let object = Object::sphere();
        let s = SimpleObject::from_object(&object).unwrap();
        let i = Intersection::new_(0., s);
        let n = s.normal_at(
            i,
            Tuple::point(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.),
        );
        assert_eq!(
            n,
            Tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)
        );
    }
}
