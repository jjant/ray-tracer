use crate::{
    intersection::Intersection, material::Material, matrix4::Matrix4, ray::Ray, tuple::Tuple,
};

pub trait Shape {
    fn transform(&self) -> Matrix4;
    fn set_transform(&mut self, transform: Matrix4);
    fn material(&self) -> Material;
    fn set_material(&mut self, material: Material);
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, local_point: Tuple) -> Tuple;

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let inverse_transform = self.transform().inverse().unwrap();
        let local_point = inverse_transform * world_point;
        let local_normal = self.local_normal_at(local_point);

        let mut world_normal = inverse_transform.transpose() * local_normal;
        // TODO: Investigate what's up with setting the w = 0;
        world_normal.w = 0.;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use std::{cell::Cell, f64::consts::PI};

    use super::*;

    struct TestShape {
        transform: Matrix4,
        material: Material,
        saved_ray: Cell<Option<Ray>>,
    }

    impl TestShape {
        fn new() -> TestShape {
            TestShape {
                transform: Matrix4::identity(),
                material: Material::new(),
                saved_ray: Cell::new(None),
            }
        }
    }

    impl Shape for TestShape {
        fn transform(&self) -> Matrix4 {
            self.transform
        }

        fn set_transform(&mut self, transform: Matrix4) {
            self.transform = transform;
        }

        fn material(&self) -> Material {
            self.material
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }

        fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
            self.saved_ray.set(Some(local_ray));

            vec![]
        }

        fn local_normal_at(&self, local_point: Tuple) -> Tuple {
            Tuple::vector(local_point.x, local_point.y, local_point.z)
        }
    }

    #[test]
    fn the_default_transformation() {
        let s = TestShape::new();

        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut s = TestShape::new();
        let t = Matrix4::translation(2., 3., 4.);

        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn the_default_material() {
        let s = TestShape::new();

        assert_eq!(s.material(), Material::new());
    }

    #[test]
    fn may_be_assigned_a_material() {
        let mut s = TestShape::new();
        let mut m = Material::new();
        m.ambient = 1.;
        s.set_material(m);

        assert_eq!(s.material(), m);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = TestShape::new();
        s.set_transform(Matrix4::scaling(2., 2., 2.));

        let xs = r.intersect(&s);

        let saved_ray = s.saved_ray.get().unwrap();
        assert_eq!(saved_ray.origin, Tuple::point(0., 0., -2.5));
        assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 0.5))
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = TestShape::new();
        s.set_transform(Matrix4::translation(5., 0., 0.));

        let xs = r.intersect(&s);

        let saved_ray = s.saved_ray.get().unwrap();
        assert_eq!(saved_ray.origin, Tuple::point(-5., 0., -5.));
        assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 1.))
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = TestShape::new();

        s.set_transform(Matrix4::translation(0., 1., 0.));

        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = TestShape::new();
        let m = Matrix4::scaling(1., 0.5, 1.) * Matrix4::rotation_z(PI / 5.);

        s.set_transform(m);

        let n = s.normal_at(Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));
        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }
}
