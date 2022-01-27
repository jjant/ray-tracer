use crate::color::Color;
use crate::intersection::Intersection;
use crate::intersection::TorUVT;
use crate::material::Material;
use crate::math::matrix4::Matrix4;
use crate::math::tuple::Tuple;
use crate::misc::EPSILON;
use crate::ray::Ray;
pub mod cone;
pub mod csg;
pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;
pub mod triangle;
use cone::Cone;
use cube::Cube;
use cylinder::Cylinder;
use plane::Plane;
use sphere::Sphere;
use triangle::Triangle;

use self::csg::Csg;

#[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(test, derive(PartialEq))]
pub struct Object {
    pub transform: Matrix4,
    pub shape: ShapeOrGroup,
}

impl Object {
    pub(crate) fn includes(&self, object: SimpleObject) -> bool {
        match &self.shape {
            ShapeOrGroup::Group(group) => group.iter().any(|o| o.includes(object)),
            ShapeOrGroup::Shape {
                shape: Shape::Csg(csg),
                ..
            } => csg.includes(object),
            ShapeOrGroup::Shape { .. } => {
                let o = SimpleObject::from_object(self).unwrap();

                o == object
            }
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let inner_bb = match &self.shape {
            ShapeOrGroup::Shape { shape, .. } => shape.bounding_box(),
            ShapeOrGroup::Group(ref group) => group
                .iter()
                .map(|object| object.bounding_box())
                .reduce(|box1, box2| BoundingBox::union(&box1, &box2))
                .unwrap(),
        };

        let new_points = inner_bb.points().map(|point| self.transform * point);

        BoundingBox::from_points(&new_points)
    }

    pub fn group(objects: Vec<Object>) -> Self {
        Object {
            transform: Matrix4::identity(),
            shape: ShapeOrGroup::Group(objects),
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self.shape {
            ShapeOrGroup::Shape {
                material: ref mut mat,
                ..
            } => {
                *mat = material;
            }
            ShapeOrGroup::Group(ref mut group) => {
                for object in group.iter_mut() {
                    object.set_material(material);
                }
            }
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let bb = self.bounding_box();
        // This is a bit different from the book, it looks like?
        // They seem to do the AABB check in the local intersect function
        // But that doesn't seem to make sense because we compute the bounding box in world space.
        let intersects_box = bb.intersect(ray);

        if intersects_box {
            let local_ray = ray.transform(self.transform.inverse().unwrap());

            self.local_intersect(local_ray)
        } else {
            vec![]
        }
    }

    fn local_intersect<'a>(&'a self, local_ray: Ray) -> Vec<Intersection<'a>> {
        match self.shape {
            ShapeOrGroup::Shape {
                shape: Shape::Csg(ref csg),
                ..
            } => csg
                .local_intersect(local_ray)
                .into_iter()
                .map(|mut i| {
                    i.object.transform = self.transform * i.object.transform;
                    i
                })
                .collect(),
            ShapeOrGroup::Group(ref group) => group
                .iter()
                .flat_map(|object| object.intersect(local_ray))
                .map(|mut i| {
                    i.object.transform = self.transform * i.object.transform;
                    i
                })
                .collect(),

            ShapeOrGroup::Shape {
                ref shape,
                ref material,
            } => shape
                .local_intersect(local_ray)
                .into_iter()
                .map(|t| {
                    Intersection::new(
                        &t,
                        SimpleObject {
                            material: *material,
                            transform: self.transform,
                            shape: &shape,
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn new(shape: Shape) -> Self {
        Self {
            transform: Matrix4::identity(),
            shape: ShapeOrGroup::Shape {
                material: Material::new(),
                shape,
            },
        }
    }

    pub fn sphere() -> Self {
        Self::new(Shape::Sphere)
    }

    pub fn plane() -> Self {
        Self::new(Shape::Plane)
    }

    pub fn cube() -> Self {
        Self::new(Shape::Cube)
    }

    pub fn cylinder() -> Self {
        Self::new(Shape::Cylinder(Cylinder::new()))
    }

    pub fn cone() -> Self {
        Self::new(Shape::Cone(Cone::new()))
    }

    pub fn union(left: Object, right: Object) -> Self {
        Self::new(Shape::Csg(Csg::union(left, right)))
    }

    pub fn intersection(left: Object, right: Object) -> Self {
        Self::new(Shape::Csg(Csg::intersection(left, right)))
    }

    pub fn difference(left: Object, right: Object) -> Self {
        Self::new(Shape::Csg(Csg::difference(left, right)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeOrGroup {
    Shape { material: Material, shape: Shape },
    Group(Vec<Object>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleObject<'a> {
    pub material: Material,
    pub transform: Matrix4,
    pub shape: &'a Shape,
}

#[derive(Debug)]
pub struct BoundingBox {
    min: Tuple,
    max: Tuple,
}

impl BoundingBox {
    #[allow(dead_code)]
    pub fn to_object(&self) -> Object {
        let Tuple {
            x: w, y: h, z: d, ..
        } = dbg!(self.max - self.min);

        let mut object = Object::cube();
        let pos = self.min + Tuple::vector(w / 2., h / 2., d / 2.);

        object.transform =
            Matrix4::translation(pos.x, pos.y, pos.z) * Matrix4::scaling(w / 2., h / 2., d / 2.);
        let mut material = Material::new();
        material.color = Color::new(0.5, 0., 0.5);
        material.transparency = 0.925;
        object.set_material(material);

        object
    }

    fn intersect(&self, world_ray: Ray) -> bool {
        cube::local_intersect(self.min, self.max, world_ray).len() > 0
    }

    pub(crate) fn from_points(points: &[Tuple]) -> BoundingBox {
        let mut min_point = Tuple::point(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max_point = Tuple::point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for point in points {
            min_point = min_point.min(point);
            max_point = max_point.max(point);
        }

        BoundingBox {
            min: min_point,
            max: max_point,
        }
    }

    fn points(&self) -> [Tuple; 8] {
        let Tuple {
            x: x_min,
            y: y_min,
            z: z_min,
            ..
        } = self.min;
        let Tuple {
            x: x_max,
            y: y_max,
            z: z_max,
            ..
        } = self.max;

        [
            Tuple::point(x_min, y_min, z_min),
            Tuple::point(x_min, y_max, z_min),
            Tuple::point(x_min, y_min, z_max),
            Tuple::point(x_min, y_max, z_max),
            Tuple::point(x_max, y_min, z_min),
            Tuple::point(x_max, y_max, z_min),
            Tuple::point(x_max, y_min, z_max),
            Tuple::point(x_max, y_max, z_max),
        ]
    }

    fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: Tuple::point(
                f64::min(self.min.x, other.min.x),
                f64::min(self.min.y, other.min.y),
                f64::min(self.min.z, other.min.z),
            ),
            max: Tuple::point(
                f64::max(self.max.x, other.max.x),
                f64::max(self.max.y, other.max.y),
                f64::max(self.max.z, other.max.z),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder(Cylinder),
    Cone(Cone),
    Triangle(Triangle),
    Csg(Csg),
}

impl Shape {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            Shape::Sphere => BoundingBox {
                min: Tuple::point(-(1. + EPSILON), -(1. + EPSILON), -(1. + EPSILON)),
                max: Tuple::point(1. + EPSILON, 1. + EPSILON, 1. + EPSILON),
            },

            Shape::Cube => BoundingBox {
                min: Tuple::point(-1., -1., -1.),
                max: Tuple::point(1., 1., 1.),
            },
            Shape::Plane => BoundingBox {
                min: Tuple::point(f64::NEG_INFINITY, 0., f64::NEG_INFINITY),
                max: Tuple::point(f64::INFINITY, 0., f64::INFINITY),
            },
            Shape::Cylinder(Cylinder {
                minimum: min_y,
                maximum: max_y,
                ..
            }) => BoundingBox {
                min: Tuple::point(-1., *min_y, -1.),
                max: Tuple::point(1., *max_y, 1.),
            },
            Shape::Cone(Cone {
                minimum: min_y,
                maximum: max_y,
                ..
            }) => {
                let max_x = f64::max(min_y.abs(), max_y.abs());
                let max_z = max_x;

                BoundingBox {
                    min: Tuple::point(-max_x, *min_y, -max_z),
                    max: Tuple::point(max_x, *max_y, max_z),
                }
            }
            Shape::Triangle(triangle) => triangle.bounding_box(),
            Shape::Csg(csg) => {
                let left = csg.left.bounding_box();
                let right = csg.right.bounding_box();

                left.union(&right)
            }
        }
    }

    pub(crate) fn local_normal_at(&self, intersection: Intersection, local_point: Tuple) -> Tuple {
        match self {
            Shape::Sphere => Sphere::local_normal_at(local_point),
            Shape::Plane => Plane::local_normal_at(local_point),
            Shape::Cube => Cube::local_normal_at(local_point),
            Shape::Cylinder(cylinder) => cylinder.local_normal_at(local_point),
            Shape::Cone(cone) => cone.local_normal_at(local_point),
            Shape::Triangle(triangle) => {
                let uvt = intersection.uvt().unwrap();

                triangle.local_normal_at(&uvt)
            }
            Shape::Csg(_) => unreachable!(),
        }
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<TorUVT> {
        match self {
            Shape::Sphere => Sphere::local_intersect(local_ray)
                .into_iter()
                .map(|t| TorUVT::JustT { t })
                .collect(),
            Shape::Plane => Plane::local_intersect(local_ray)
                .into_iter()
                .map(|t| TorUVT::JustT { t })
                .collect(),
            Shape::Cube => Cube::local_intersect(local_ray)
                .into_iter()
                .map(|t| TorUVT::JustT { t })
                .collect(),
            Shape::Cylinder(cylinder) => cylinder
                .local_intersect(local_ray)
                .into_iter()
                .map(|t| TorUVT::JustT { t })
                .collect(),
            Shape::Cone(cone) => cone
                .local_intersect(local_ray)
                .into_iter()
                .map(|t| TorUVT::JustT { t })
                .collect(),
            Shape::Triangle(triangle) => triangle
                .local_intersect(local_ray)
                .into_iter()
                .map(|uvt| TorUVT::UVT { uvt })
                .collect(),
            Shape::Csg(_) => unreachable!(),
        }
    }
}

impl<'a> SimpleObject<'a> {
    pub(crate) fn from_object(object: &'a Object) -> Option<Self> {
        match &object.shape {
            ShapeOrGroup::Shape { material, shape } => Some(Self {
                transform: object.transform,
                material: *material,
                shape: shape,
            }),
            ShapeOrGroup::Group(_) => None,
        }
    }

    pub fn transform(&self) -> Matrix4 {
        self.transform
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn normal_at(&self, intersection: Intersection, world_point: Tuple) -> Tuple {
        let inverse_transform = self.transform().inverse().unwrap();
        let local_point = inverse_transform * world_point;
        let local_normal = self.shape.local_normal_at(intersection, local_point);

        let mut world_normal = inverse_transform.transpose() * local_normal;
        // TODO: Investigate what's up with setting the w = 0;
        world_normal.w = 0.;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::tuple::Tuple;
    use std::f64::consts::PI;

    use super::*;

    impl Object {
        pub(crate) fn glass_sphere() -> Self {
            let mut s = Self::sphere();
            let mut material = Material::new();
            material.transparency = 1.0;
            material.refractive_index = 1.5;
            s.set_material(material);

            s
        }
    }

    impl<'a> SimpleObject<'a> {
        /// The maths assume the sphere is located in the origin,
        /// and it handles the general case by "unmoving" the ray with the opposite transform.
        pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
            let local_ray = ray.transform(self.transform().inverse().unwrap());

            self.shape
                .local_intersect(local_ray)
                .into_iter()
                .map(|t_or_uvt| Intersection::new(&t_or_uvt, *self))
                .collect()
        }
    }

    #[test]
    fn the_default_transformation() {
        let object = Object::sphere();
        let s = SimpleObject::from_object(&object).unwrap();

        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut object = Object::sphere();
        let t = Matrix4::translation(2., 3., 4.);
        object.transform = t;
        let s = SimpleObject::from_object(&object).unwrap();

        assert_eq!(s.transform, t);
    }

    #[test]
    fn the_default_material() {
        let object = Object::sphere();
        let s = SimpleObject::from_object(&object).unwrap();

        assert_eq!(s.material(), Material::new());
    }

    #[test]
    fn may_be_assigned_a_material() {
        let mut object = Object::sphere();
        let mut m = Material::new();
        m.ambient = 1.;
        object.set_material(m);
        let s = SimpleObject::from_object(&object).unwrap();

        assert_eq!(s.material(), m);
    }

    // #[test]
    // fn intersecting_a_scaled_shape_with_a_ray() {
    //     let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
    //     let mut s = SimpleObject::new(Shape::Sphere);
    //     s.set_transform(Matrix4::scaling(2., 2., 2.));

    //     let xs = s.intersect(r);

    //     let saved_ray = s.saved_ray.get().unwrap();
    //     assert_eq!(saved_ray.origin, Tuple::point(0., 0., -2.5));
    //     assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 0.5))
    // }

    // #[test]
    // fn intersecting_a_translated_shape_with_a_ray() {
    //     let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
    //     let mut s = SimpleObject::new(Shape::Sphere);
    //     s.set_transform(Matrix4::translation(5., 0., 0.));

    //     let xs = s.intersect(r);

    //     let saved_ray = s.saved_ray.get().unwrap();
    //     assert_eq!(saved_ray.origin, Tuple::point(-5., 0., -5.));
    //     assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 1.))
    // }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut object = Object::sphere();
        object.transform = Matrix4::translation(0., 1., 0.);
        let s = SimpleObject::from_object(&object).unwrap();

        let i = Intersection::new_(0., s);
        let n = s.normal_at(i, Tuple::point(0., 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut object = Object::sphere();
        let transform = Matrix4::scaling(1., 0.5, 1.) * Matrix4::rotation_z(PI / 5.);
        object.transform = transform;
        let s = SimpleObject::from_object(&object).unwrap();

        let i = Intersection::new_(0., s);
        let n = s.normal_at(i, Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));
        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let object = Object::glass_sphere();
        let s = SimpleObject::from_object(&object).unwrap();

        assert_eq!(s.transform, Matrix4::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}
