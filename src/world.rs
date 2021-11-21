use crate::color::Color;
use crate::intersection::{ComputedIntersection, Intersection};
use crate::light::Light;
use crate::material;
use crate::ray::Ray;
use crate::shape::Object;
use crate::tuple::Tuple;
pub struct World {
    pub objects: Vec<Object>,
    pub light: Option<Light>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light: None,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .collect();

        intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());

        intersections
    }

    pub fn shade_hit(&self, comps: ComputedIntersection) -> Color {
        material::lighting(
            comps.object.material(),
            comps.object,
            self.light
                .expect("Expected light to be present in shade_hit"),
            comps.point,
            comps.eye_vector,
            comps.normal_vector,
            self.is_shadowed(comps.over_point),
        )
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        if let Some(light) = self.light {
            let vector = light.position - point;
            let distance = vector.magnitude();

            let ray = Ray::new(point, vector.normalize());

            Intersection::hit(&self.intersect(ray))
                // Check to see if hit object is closer than the light.
                .map(|hit| hit.t < distance)
                .unwrap_or(false)
        } else {
            true
        }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect(ray);
        let hit = Intersection::hit(&intersections);

        if let Some(i) = hit {
            self.shade_hit(i.prepare_computations(ray))
        } else {
            Color::black()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix4::Matrix4;
    use crate::misc::approx_equal;

    impl World {
        pub fn default() -> Self {
            let mut s1 = Object::sphere();
            {
                let material = s1.material_mut();
                material.color = Color::new(0.8, 1.0, 0.6);
                material.diffuse = 0.7;
                material.specular = 0.2;
            }

            let mut s2 = Object::sphere();
            *s2.transform_mut() = Matrix4::scaling(0.5, 0.5, 0.5);

            Self {
                objects: vec![s1, s2],
                light: Some(Light::point_light(
                    Tuple::point(-10., 10., -10.),
                    Color::white(),
                )),
            }
        }
    }

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    #[test]
    fn the_default_world() {
        let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::white());
        let mut s1 = Object::sphere();
        {
            let mut material = s1.material_mut();
            material.color = Color::new(0.8, 1.0, 0.6);
            material.diffuse = 0.7;
            material.specular = 0.2;
        }

        let mut s2 = Object::sphere();
        *s2.transform_mut() = Matrix4::scaling(0.5, 0.5, 0.5);

        let w = World::default();

        assert_eq!(w.light, Some(light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
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
        let shape = w.objects[0];
        let i = Intersection::new(4., shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Some(Light::point_light(
            Tuple::point(0., 0.25, 0.),
            Color::white(),
        ));

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

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
        {
            let outer = &mut w.objects[0];
            outer.material_mut().ambient = 1.;
            let inner = &mut w.objects[1];
            inner.material_mut().ambient = 1.;
        }

        let inner = &w.objects[1];
        let r = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));
        let c = w.color_at(r);

        assert_eq!(c, inner.material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0., 10., 0.);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Tuple::point(10., -10., 10.);
        assert!(w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20., 20., -20.);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2., 2., -2.);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(Light::point_light(
            Tuple::point(0., 0., -10.),
            Color::new(1., 1., 1.),
        ));

        let s1 = Object::sphere();
        w.objects.push(s1);
        let mut s2 = Object::sphere();
        *s2.transform_mut() = Matrix4::translation(0., 0., 10.);
        w.objects.push(s2);

        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let i = Intersection::new(4., s2);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
