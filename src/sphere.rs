use crate::material::Material;
use crate::matrix4::Matrix4;
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

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform
    }

    pub fn normal_at(self, world_point: Tuple) -> Tuple {
        let inverse_transform = self.transform.inverse().unwrap();
        let object_point = inverse_transform * world_point;

        let object_normal = object_point - Tuple::point(0., 0., 0.);

        let mut world_normal = inverse_transform.transpose() * object_normal;
        // TODO: Investigate what's up with setting the w = 0;
        world_normal.w = 0.;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Matrix4::translation(2., 3., 4.);

        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

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

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();

        s.set_transform(Matrix4::translation(0., 1., 0.));

        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix4::scaling(1., 0.5, 1.) * Matrix4::rotation_z(PI / 5.);

        s.set_transform(m);

        let n = s.normal_at(Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));
        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();

        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.;
        s.material = m;

        assert_eq!(s.material, m);
    }
}
