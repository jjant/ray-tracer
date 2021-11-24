use crate::misc::{approx_equal, EPSILON};
use crate::ray::Ray;
use crate::shape::Object;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
        Self { t, object }
    }

    // Returns the closest intersection, that is
    // the one with the smallest non-negative t value.
    pub fn hit(intersections: &[Self]) -> Option<Self> {
        intersections
            .iter()
            .filter(|i| (**i).t >= 0.)
            .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap())
            .copied()
    }

    pub fn prepare_computations(self, ray: Ray) -> ComputedIntersection {
        let object = self.object;
        let t = self.t;
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;

        let tentative_normal = self.object.normal_at(point);

        let (inside, normal_vector) = if tentative_normal.dot(eye_vector) < 0. {
            (true, -tentative_normal)
        } else {
            (false, tentative_normal)
        };

        let reflect_vector = ray.direction.reflect(normal_vector);
        let over_point = point + normal_vector * EPSILON;

        ComputedIntersection {
            object,
            t,
            point,
            eye_vector,
            normal_vector,
            reflect_vector,
            inside,
            over_point,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.t, other.t) && self.object == other.object
    }
}

pub struct ComputedIntersection {
    pub t: f64,
    pub object: Object,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub reflect_vector: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

#[cfg(test)]
mod tests {
    use crate::matrix4::Matrix4;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Object::sphere();
        let i = Intersection::new(3.5, s);

        assert!(approx_equal(i.t, 3.5));
        assert_eq!(i.object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(2., s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(-1., s);
        let i2 = Intersection::new(1., s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(-2., s);
        let i2 = Intersection::new(-1., s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Object::sphere();
        let i1 = Intersection::new(5., s);
        let i2 = Intersection::new(7., s);
        let i3 = Intersection::new(-3., s);
        let i4 = Intersection::new(2., s);
        let xs = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Object::sphere();
        let intersection = Intersection::new(4., shape);

        let comps = intersection.prepare_computations(r);

        assert!(approx_equal(comps.t, intersection.t));
        assert_eq!(comps.object, intersection.object);
        assert_eq!(comps.point, Tuple::point(0., 0., -1.));
        assert_eq!(comps.eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(comps.normal_vector, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Object::sphere();
        let i = Intersection::new(4., shape);
        let comps = i.prepare_computations(r);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Object::sphere();
        let i = Intersection::new(1., shape);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Tuple::point(0., 0., 1.));
        assert_eq!(comps.eye_vector, Tuple::vector(0., 0., -1.));
        assert!(comps.inside);
        // Normal would have been (0., 0., 1.), but is inverted!
        assert_eq!(comps.normal_vector, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Object::sphere();
        *shape.transform_mut() = Matrix4::translation(0., 0., 1.);
        let i = Intersection::new(5., shape);
        let comps = i.prepare_computations(r);

        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Object::plane();
        let r = Ray::new(
            Tuple::point(0., 1., -1.),
            Tuple::vector(0., -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64),
        );
        let i = Intersection::new(2_f64.sqrt(), shape);
        let comps = i.prepare_computations(r);

        assert_eq!(
            comps.reflect_vector,
            Tuple::vector(0., 2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64)
        );
    }
}
