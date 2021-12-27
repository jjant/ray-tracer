use crate::cone::Cone;
use crate::cylinder::Cylinder;
use crate::plane::Plane;
use crate::{
    cube::Cube, intersection::Intersection, material::Material, matrix4::Matrix4, ray::Ray,
    sphere::Sphere, tuple::Tuple,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Object {
    material: Material,
    transform: Matrix4,
    shape: Shape,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder(Cylinder),
    Cone(Cone),
}

impl Shape {
    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        match self {
            Shape::Sphere => Sphere::local_normal_at(local_point),
            Shape::Plane => Plane::local_normal_at(local_point),
            Shape::Cube => Cube::local_normal_at(local_point),
            Shape::Cylinder(cylinder) => cylinder.local_normal_at(local_point),
            Shape::Cone(cone) => cone.local_normal_at(local_point),
        }
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<f64> {
        match self {
            Shape::Sphere => Sphere::local_intersect(local_ray),
            Shape::Plane => Plane::local_intersect(local_ray),
            Shape::Cube => Cube::local_intersect(local_ray),
            Shape::Cylinder(cylinder) => cylinder.local_intersect(local_ray),
            Shape::Cone(cone) => cone.local_intersect(local_ray),
        }
    }
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            transform: Matrix4::identity(),
            material: Material::new(),
            shape: shape,
        }
    }

    /// TODO: Document
    #[allow(dead_code)]
    pub fn sphere() -> Self {
        Self::new(Shape::Sphere)
    }

    /// TODO: Document (specially that it defaults to an XZ plane)
    #[allow(dead_code)]
    pub fn plane() -> Self {
        Self::new(Shape::Plane)
    }

    pub fn cube() -> Self {
        Self::new(Shape::Cube)
    }

    #[allow(dead_code)]
    pub fn cylinder() -> Self {
        Self::new(Shape::Cylinder(Cylinder::new()))
    }

    #[allow(dead_code)]
    pub fn cone() -> Self {
        Self::new(Shape::Cone(Cone::new()))
    }

    pub fn transform(&self) -> Matrix4 {
        self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Matrix4 {
        &mut self.transform
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    /// The maths assume the sphere is located in the origin,
    /// and it handles the general case by "unmoving" the ray with the opposite transform.
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transform().inverse().unwrap());

        self.shape
            .local_intersect(local_ray)
            .into_iter()
            .map(|t| Intersection::new(t, *self))
            .collect()
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let inverse_transform = self.transform().inverse().unwrap();
        let local_point = inverse_transform * world_point;
        let local_normal = self.shape.local_normal_at(local_point);

        let mut world_normal = inverse_transform.transpose() * local_normal;
        // TODO: Investigate what's up with setting the w = 0;
        world_normal.w = 0.;

        world_normal.normalize()
    }

    #[allow(dead_code)]
    pub fn glass_sphere() -> Self {
        let mut s = Self::sphere();

        s.material.transparency = 1.0;
        s.material.refractive_index = 1.5;

        s
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn the_default_transformation() {
        let s = Object::new(Shape::Sphere);

        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut s = Object::new(Shape::Sphere);
        let t = Matrix4::translation(2., 3., 4.);

        *s.transform_mut() = t;

        assert_eq!(s.transform, t);
    }

    #[test]
    fn the_default_material() {
        let s = Object::new(Shape::Sphere);

        assert_eq!(s.material(), Material::new());
    }

    #[test]
    fn may_be_assigned_a_material() {
        let mut s = Object::new(Shape::Sphere);
        let mut m = Material::new();
        m.ambient = 1.;

        *s.material_mut() = m;

        assert_eq!(s.material(), m);
    }

    // #[test]
    // fn intersecting_a_scaled_shape_with_a_ray() {
    //     let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
    //     let mut s = Object::new(Shape::Sphere);
    //     s.set_transform(Matrix4::scaling(2., 2., 2.));

    //     let xs = s.intersect(r);

    //     let saved_ray = s.saved_ray.get().unwrap();
    //     assert_eq!(saved_ray.origin, Tuple::point(0., 0., -2.5));
    //     assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 0.5))
    // }

    // #[test]
    // fn intersecting_a_translated_shape_with_a_ray() {
    //     let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
    //     let mut s = Object::new(Shape::Sphere);
    //     s.set_transform(Matrix4::translation(5., 0., 0.));

    //     let xs = s.intersect(r);

    //     let saved_ray = s.saved_ray.get().unwrap();
    //     assert_eq!(saved_ray.origin, Tuple::point(-5., 0., -5.));
    //     assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 1.))
    // }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = Object::new(Shape::Sphere);

        *s.transform_mut() = Matrix4::translation(0., 1., 0.);

        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = Object::new(Shape::Sphere);
        let m = Matrix4::scaling(1., 0.5, 1.) * Matrix4::rotation_z(PI / 5.);

        *s.transform_mut() = m;

        let n = s.normal_at(Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));
        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = Object::glass_sphere();

        assert_eq!(s.transform, Matrix4::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}
