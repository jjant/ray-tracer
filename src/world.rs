use crate::color::Color;
use crate::intersection::{ComputedIntersection, Intersection};
use crate::light::Light;
use crate::material;
use crate::ray::Ray;
use crate::shape::{Object, SimpleObject};
use crate::tuple::Tuple;

const DEFAULT_ALLOWED_DEPTH: i32 = 8;

pub struct World {
    pub objects: Vec<Object>,
    lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    pub fn add_group(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn add_object(&mut self, object: SimpleObject) {
        self.objects.push(Object::from_simple(object))
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        self.color_at_with_depth(ray, DEFAULT_ALLOWED_DEPTH)
    }

    pub fn color_at_with_depth(&self, ray: Ray, remaining_depth: i32) -> Color {
        let intersections = self.intersect(ray);
        let hit = Intersection::hit(&intersections);

        if let Some(i) = hit {
            self.shade_hit(i.prepare_computations(ray, &intersections), remaining_depth)
        } else {
            Color::black()
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .collect();

        intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());

        intersections
    }

    fn shade_hit(&self, comps: ComputedIntersection, remaining_depth: i32) -> Color {
        let surface_color = self
            .lights
            .iter()
            .map(|light| {
                material::lighting(
                    comps.object.material(),
                    comps.object,
                    *light,
                    // Use comps.over_point instead of comps.point remove acne from floor with checkered pattern.
                    // See https://forum.raytracerchallenge.com/thread/204/avoid-noise-checkers-pattern-planes
                    comps.over_point,
                    comps.eye_vector,
                    comps.normal_vector,
                    self.is_shadowed(comps.over_point, *light),
                )
            })
            .fold(Color::black(), |c1, c2| c1 + c2);

        let reflected_color = self.reflected_color(comps, remaining_depth);
        let refracted_color = self.refracted_color(comps, remaining_depth);

        let material = comps.object.material();

        if material.reflective > 0. && material.transparency > 0. {
            let reflectance = comps.schlick();

            surface_color + reflected_color * reflectance + refracted_color * (1. - reflectance)
        } else {
            surface_color + reflected_color + refracted_color
        }
    }

    fn is_shadowed(&self, point: Tuple, light: Light) -> bool {
        let vector = light.position - point;
        let distance = vector.magnitude();

        let ray = Ray::new(point, vector.normalize());

        Intersection::hit(&self.intersect(ray))
            // Check to see if hit object is closer than the light.
            .map(|hit| hit.t < distance)
            .unwrap_or(false)
    }

    fn reflected_color(&self, comps: ComputedIntersection, remaining_depth: i32) -> Color {
        let no_depth_remaining = remaining_depth <= 0;
        let default_color = Color::black();

        if no_depth_remaining {
            return default_color;
        }
        let reflective = comps.object.material().reflective;
        if reflective > 0. {
            let reflect_ray = Ray::new(comps.over_point, comps.reflect_vector);
            let color = self.color_at_with_depth(reflect_ray, remaining_depth - 1);

            color * reflective
        } else {
            default_color
        }
    }

    fn refracted_color(&self, comps: ComputedIntersection, remaining_depth: i32) -> Color {
        let object_is_opaque = comps.object.material().transparency == 0.;
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_vector.dot(comps.normal_vector);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        let total_internal_reflection = sin2_t > 1.;

        if remaining_depth == 0 || object_is_opaque || total_internal_reflection {
            Color::black()
        } else {
            let cos_t = (1. - sin2_t).sqrt();
            let direction =
                comps.normal_vector * (n_ratio * cos_i - cos_t) - comps.eye_vector * n_ratio;

            let refract_ray = Ray::new(comps.under_point, direction);

            let color = self.color_at_with_depth(refract_ray, remaining_depth - 1)
                * comps.object.material().transparency;

            color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::matrix4::Matrix4;
    use crate::misc::approx_equal;
    use crate::pattern::Pattern;
    use crate::shape::ShapeOrGroup;

    impl World {
        pub fn default() -> Self {
            let mut s1 = SimpleObject::sphere();
            s1.material_mut().color = Color::new(0.8, 1.0, 0.6);
            s1.material_mut().diffuse = 0.7;
            s1.material_mut().specular = 0.2;

            let mut s2 = SimpleObject::sphere();
            *s2.transform_mut() = Matrix4::scaling(0.5, 0.5, 0.5);

            let mut world = Self::new();
            world.add_object(s1);
            world.add_object(s2);
            world.add_light(Light::point_light(
                Tuple::point(-10., 10., -10.),
                Color::white(),
            ));

            world
        }

        fn get_object(&self, index: usize) -> Option<SimpleObject> {
            match self.objects.get(index) {
                Some(Object {
                    transform,
                    shape: ShapeOrGroup::Shape { shape, material },
                }) => Some(SimpleObject {
                    material: *material,
                    transform: *transform,
                    shape: *shape,
                }),
                Some(Object {
                    shape: ShapeOrGroup::Group(_),
                    ..
                }) => None,

                None => None,
            }
        }

        fn is_empty(&self) -> bool {
            self.objects.is_empty()
        }
    }

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.is_empty());
        assert!(w.lights.is_empty());
    }

    #[test]
    fn the_default_world() {
        let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::white());
        let mut s1 = SimpleObject::sphere();
        s1.material_mut().color = Color::new(0.8, 1.0, 0.6);
        s1.material_mut().diffuse = 0.7;
        s1.material_mut().specular = 0.2;

        let mut s2 = SimpleObject::sphere();
        *s2.transform_mut() = Matrix4::scaling(0.5, 0.5, 0.5);

        let w = World::default();

        assert_eq!(w.lights, vec![light]);
        assert!(w.get_object(0).unwrap() == s1);
        assert!(w.get_object(1).unwrap() == s2);
        // TODO: See if there's a good way of implementing this.
        // assert!(w.contains(&s1));
        // assert!(w.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = dbg!(w.intersect(r));

        assert_eq!(xs.len(), 4);
        assert!(approx_equal(xs[0].t, 4.));
        assert!(approx_equal(xs[1].t, 4.5));
        assert!(approx_equal(xs[2].t, 5.5));
        assert!(approx_equal(xs[3].t, 6.));
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = w.get_object(0).unwrap();
        let i = Intersection::new_(4., shape);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.lights[0] = Light::point_light(Tuple::point(0., 0.25, 0.), Color::white());

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = w.get_object(1).unwrap();
        let i = Intersection::new_(0.5, shape);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));
        let c = w.color_at(r);

        assert_eq!(c, Color::black())
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        // TODO: See if we can refactor this
        let mut w = World::default();
        let outer = &mut w.objects[0];
        let mut material = Material::new();
        material.ambient = 1.;
        outer.set_material(material);
        let inner = &mut w.objects[1];
        let mut material = Material::new();
        material.ambient = 1.;
        inner.set_material(material);

        let inner = w.get_object(1).unwrap();
        let r = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));
        let c = w.color_at(r);

        assert_eq!(c, inner.material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0., 10., 0.);
        assert!(!w.is_shadowed(p, w.lights[0]));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Tuple::point(10., -10., 10.);
        assert!(w.is_shadowed(p, w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20., 20., -20.);
        assert!(!w.is_shadowed(p, w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2., 2., -2.);
        assert!(!w.is_shadowed(p, w.lights[0]));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.add_light(Light::point_light(
            Tuple::point(0., 0., -10.),
            Color::new(1., 1., 1.),
        ));

        let s1 = SimpleObject::sphere();
        w.add_object(s1);
        let mut s2 = SimpleObject::sphere();
        *s2.transform_mut() = Matrix4::translation(0., 0., 10.);
        w.add_object(s2);

        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let i = Intersection::new_(4., s2);
        let comps = i.prepare_computations(r, &[i]);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = &mut w.objects[0];
        let mut material = Material::new();
        material.ambient = 1.;
        shape.set_material(material);
        let shape = w.get_object(0).unwrap();

        let i = Intersection::new_(1., shape);
        let comps = i.prepare_computations(r, &[i]);
        let color = w.reflected_color(comps, 5);

        assert_eq!(color, Color::new(0., 0., 0.))
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let mut shape = SimpleObject::plane();
        shape.material_mut().reflective = 0.5;
        *shape.transform_mut() = Matrix4::translation(0., -1., 0.);
        w.add_object(shape);

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new_(2_f64.sqrt(), shape);
        let comps = i.prepare_computations(r, &[i]);
        let color = w.reflected_color(comps, 5);

        assert_eq!(color, Color::new(0.19033, 0.23791, 0.142747));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::default();
        let mut shape = SimpleObject::plane();
        shape.material_mut().reflective = 0.5;
        *shape.transform_mut() = Matrix4::translation(0., -1., 0.);
        w.add_object(shape);
        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new_(2_f64.sqrt(), shape);
        let comps = i.prepare_computations(r, &[i]);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.add_light(Light::point_light(
            Tuple::point(0., 0., 0.),
            Color::new(1., 1., 1.),
        ));

        let mut lower = SimpleObject::plane();
        lower.material_mut().reflective = 1.;
        *lower.transform_mut() = Matrix4::translation(0., -1., 0.);
        w.add_object(lower);

        let mut upper = SimpleObject::plane();
        upper.material_mut().reflective = 1.;
        *upper.transform_mut() = Matrix4::translation(0., 1., 0.);
        w.add_object(upper);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));

        let _c = w.color_at(r);

        assert!(true);
    }
    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let mut shape = SimpleObject::plane();
        shape.material_mut().reflective = 0.5;
        *shape.transform_mut() = Matrix4::translation(0., -1., 0.);
        w.add_object(shape);
        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new_(2_f64.sqrt(), shape);
        let comps = i.prepare_computations(r, &[i]);
        let color = w.reflected_color(comps, 0);

        assert_eq!(color, Color::black());
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.get_object(0).unwrap();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = [Intersection::new_(4., shape), Intersection::new_(6., shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let shape = &mut w.objects[0];
        let mut material = Material::new();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        shape.set_material(material);
        let shape = w.get_object(0).unwrap();

        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = [Intersection::new_(4., shape), Intersection::new_(6., shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 0);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        let shape = &mut w.objects[0];
        let mut material = Material::new();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        shape.set_material(material);
        let shape = w.get_object(0).unwrap();

        let r = Ray::new(
            Tuple::point(0., 0., 2_f64.sqrt() / 2.),
            Tuple::vector(0., 1., 0.),
        );

        let xs = vec![
            Intersection::new_(-2_f64.sqrt() / 2., shape),
            Intersection::new_(2_f64.sqrt() / 2., shape),
        ];

        // NOTE: this time you're inside the sphere, so you need
        // to look at the second intersection, xs[1], not xs[0]
        let comps = xs[1].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = World::default();

        let a = &mut w.objects[0];
        let mut material = Material::with_pattern(Pattern::test());
        material.ambient = 1.0;
        a.set_material(material);

        let b = &mut w.objects[1];
        let mut material = Material::new();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        b.set_material(material);

        let a = w.get_object(0).unwrap();
        let b = w.get_object(1).unwrap();

        let r = Ray::new(Tuple::point(0., 0., 0.1), Tuple::vector(0., 1., 0.));
        let xs = vec![
            Intersection::new_(-0.9899, a),
            Intersection::new_(-0.4899, b),
            Intersection::new_(0.4899, b),
            Intersection::new_(0.9899, a),
        ];
        let comps = xs[2].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Color::new(0., 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = World::default();

        let mut floor = SimpleObject::plane();
        *floor.transform_mut() = Matrix4::translation(0., -1., 0.);
        floor.material_mut().transparency = 0.5;
        floor.material_mut().refractive_index = 1.5;
        w.add_object(floor);

        let mut ball = SimpleObject::sphere();
        ball.material_mut().color = Color::new(1., 0., 0.);
        ball.material_mut().ambient = 0.5;
        *ball.transform_mut() = Matrix4::translation(0., -3.5, -0.5);
        w.add_object(ball);

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
        );
        let xs = vec![Intersection::new_(2_f64.sqrt(), floor)];
        let comps = xs[0].prepare_computations(r, &xs);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = World::default();
        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
        );

        let mut floor = SimpleObject::plane();
        *floor.transform_mut() = Matrix4::translation(0., -1., 0.);
        floor.material_mut().reflective = 0.5;
        floor.material_mut().transparency = 0.5;
        floor.material_mut().refractive_index = 1.5;
        w.add_object(floor);

        let mut ball = SimpleObject::sphere();
        ball.material_mut().color = Color::new(1., 0., 0.);
        ball.material_mut().ambient = 0.5;
        *ball.transform_mut() = Matrix4::translation(0., -3.5, -0.5);
        w.add_object(ball);

        let xs = vec![Intersection::new_(2_f64.sqrt(), floor)];
        let comps = xs[0].prepare_computations(r, &xs);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
