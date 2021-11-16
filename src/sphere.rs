use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix4::Matrix4;
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }
}

impl Shape for Sphere {
    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform
    }

    fn material(&self) -> Material {
        self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(
        &self,
        local_ray: crate::ray::Ray,
    ) -> Vec<crate::intersection::Intersection> {
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

            vec![Intersection::new(t1, *self), Intersection::new(t2, *self)]
        }
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        // Warning: do not remove this (consider the w!)
        local_point - Tuple::point(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(1., 0., 0.));
        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0., 1., 0.));
        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0., 0., 1.));
        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
        ));
        assert_eq!(
            n,
            Tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)
        );
    }
}
