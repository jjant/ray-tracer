use crate::misc::{approx_equal, EPSILON};
use crate::ray::Ray;
use crate::shape::SimpleObject;
use crate::triangle::UVT;
use crate::tuple::Tuple;

pub(crate) enum TorUVT {
    JustT { t: f64 },
    UVT { uvt: UVT },
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: f64,
    uv: Option<(f64, f64)>,
    pub object: SimpleObject,
}

impl Intersection {
    pub(crate) fn new(t_or_uvt: &TorUVT, object: SimpleObject) -> Self {
        match t_or_uvt {
            &TorUVT::JustT { t } => Self {
                t,
                uv: None,
                object,
            },
            &TorUVT::UVT { uvt } => Self {
                t: uvt.t,
                uv: Some((uvt.u, uvt.v)),
                object,
            },
        }
    }

    // Returns the closest intersection, that is
    // the one with the smallest non-negative t value.
    pub fn hit(intersections: &[Self]) -> Option<&Self> {
        intersections
            .iter()
            .filter(|i| (**i).t >= 0.)
            .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap())
    }

    pub fn prepare_computations(
        &self,
        ray: Ray,
        all_intersections: &[Intersection],
    ) -> ComputedIntersection {
        let object = self.object;
        let t = self.t;
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;

        let tentative_normal = self.object.normal_at(self, point);

        let (inside, normal_vector) = if tentative_normal.dot(eye_vector) < 0. {
            (true, -tentative_normal)
        } else {
            (false, tentative_normal)
        };

        let reflect_vector = ray.direction.reflect(normal_vector);
        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;

        let (n1, n2) = self.compute_refractive_indices(all_intersections);

        ComputedIntersection {
            object,
            t,
            point,
            eye_vector,
            normal_vector,
            reflect_vector,
            inside,
            over_point,
            under_point,
            n1: n1,
            n2: n2,
        }
    }

    fn compute_refractive_indices(&self, all_intersections: &[Intersection]) -> (f64, f64) {
        let mut containers: Vec<SimpleObject> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        for i in all_intersections {
            // Bad phrasing by the author, check this:
            // https://forum.raytracerchallenge.com/post/103/thread
            let is_hit = i == self;

            if is_hit {
                if let Some(last) = containers.last() {
                    n1 = last.material().refractive_index;
                } else {
                    n1 = 1.0;
                }
            }

            let position = containers.iter().position(|o| *o == i.object);

            if let Some(index) = position {
                containers.remove(index);
            } else {
                containers.push(i.object)
            }

            if is_hit {
                if let Some(last) = containers.last() {
                    n2 = last.material().refractive_index;
                } else {
                    n2 = 1.0;
                }
                break;
            }
        }

        (n1, n2)
    }

    pub(crate) fn uvt(&self) -> Option<UVT> {
        self.uv.map(|(u, v)| UVT { t: self.t, u, v })
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.t, other.t) && self.object == other.object
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComputedIntersection {
    pub t: f64,
    pub object: SimpleObject,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub reflect_vector: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl ComputedIntersection {
    pub fn schlick(&self) -> f64 {
        // find the cosine of the angle between the eye and normal vectors
        let mut cos = self.eye_vector.dot(self.normal_vector);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }
            // compute cosine of theta_t using trig identity
            let cos_t = (1. - sin2_t).sqrt();

            // when n1 > n2, use cos(theta_t) instead
            cos = cos_t
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        return r0 + (1. - r0) * (1. - cos).powi(5);
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix4::Matrix4;

    use super::*;

    impl Intersection {
        pub fn new_(t: f64, object: SimpleObject) -> Self {
            Self::new(&TorUVT::JustT { t }, object)
        }
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = SimpleObject::sphere();
        let i = Intersection::new(&TorUVT::JustT { t: 3.5 }, s);

        assert!(approx_equal(i.t, 3.5));
        assert_eq!(i.object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = SimpleObject::sphere();
        let i1 = Intersection::new(&TorUVT::JustT { t: 1. }, s);
        let i2 = Intersection::new(&TorUVT::JustT { t: 2. }, s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = SimpleObject::sphere();
        let i1 = Intersection::new(&TorUVT::JustT { t: -1. }, s);
        let i2 = Intersection::new(&TorUVT::JustT { t: 1. }, s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = SimpleObject::sphere();
        let i1 = Intersection::new(&TorUVT::JustT { t: -2. }, s);
        let i2 = Intersection::new(&TorUVT::JustT { t: -1. }, s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = SimpleObject::sphere();
        let i1 = Intersection::new(&TorUVT::JustT { t: 5. }, s);
        let i2 = Intersection::new(&TorUVT::JustT { t: 7. }, s);
        let i3 = Intersection::new(&TorUVT::JustT { t: -3. }, s);
        let i4 = Intersection::new(&TorUVT::JustT { t: 2. }, s);
        let xs = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&xs);

        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = SimpleObject::sphere();
        let intersection = Intersection::new(&TorUVT::JustT { t: 4. }, shape);

        let comps = intersection.prepare_computations(r, &[intersection]);

        assert!(approx_equal(comps.t, intersection.t));
        assert_eq!(comps.object, intersection.object);
        assert_eq!(comps.point, Tuple::point(0., 0., -1.));
        assert_eq!(comps.eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(comps.normal_vector, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = SimpleObject::sphere();
        let i = Intersection::new(&TorUVT::JustT { t: 4. }, shape);
        let comps = i.prepare_computations(r, &[i]);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = SimpleObject::sphere();
        let i = Intersection::new(&TorUVT::JustT { t: 1. }, shape);
        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(comps.point, Tuple::point(0., 0., 1.));
        assert_eq!(comps.eye_vector, Tuple::vector(0., 0., -1.));
        assert!(comps.inside);
        // Normal would have been (0., 0., 1.), but is inverted!
        assert_eq!(comps.normal_vector, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = SimpleObject::sphere();
        *shape.transform_mut() = Matrix4::translation(0., 0., 1.);
        let i = Intersection::new(&TorUVT::JustT { t: 5. }, shape);
        let comps = i.prepare_computations(r, &[i]);

        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = SimpleObject::plane();
        let r = Ray::new(
            Tuple::point(0., 1., -1.),
            Tuple::vector(0., -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64),
        );
        let i = Intersection::new(&TorUVT::JustT { t: 2_f64.sqrt() }, shape);
        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(
            comps.reflect_vector,
            Tuple::vector(0., 2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = SimpleObject::glass_sphere();
        *a.transform_mut() = Matrix4::scaling(2., 2., 2.);
        a.material_mut().refractive_index = 1.5;

        let mut b = SimpleObject::glass_sphere();
        *b.transform_mut() = Matrix4::translation(0., 0., -0.25);
        b.material_mut().refractive_index = 2.0;

        let mut c = SimpleObject::glass_sphere();
        *c.transform_mut() = Matrix4::translation(0., 0., 0.25);
        c.material_mut().refractive_index = 2.5;

        let ray = Ray::new(Tuple::point(0., 0., -4.), Tuple::vector(0., 0., 1.));
        let intersections_with_expected_indices = vec![
            (Intersection::new(&TorUVT::JustT { t: 2.0 }, a), 1.0, 1.5),
            (Intersection::new(&TorUVT::JustT { t: 2.75 }, b), 1.5, 2.0),
            (Intersection::new(&TorUVT::JustT { t: 3.25 }, c), 2.0, 2.5),
            (Intersection::new(&TorUVT::JustT { t: 4.75 }, b), 2.5, 2.5),
            (Intersection::new(&TorUVT::JustT { t: 5.25 }, c), 2.5, 1.5),
            (Intersection::new(&TorUVT::JustT { t: 6.0 }, a), 1.5, 1.0),
        ];

        let xs = intersections_with_expected_indices
            .iter()
            .map(|(i, _, _)| i)
            .copied()
            .collect::<Vec<_>>();

        let computed_intersections = intersections_with_expected_indices
            .iter()
            .map(|(intersection, n1, n2)| (intersection.prepare_computations(ray, &xs), n1, n2))
            .collect::<Vec<_>>();

        for (comps, n1, n2) in computed_intersections {
            assert!(approx_equal(comps.n1, *n1));
            assert!(approx_equal(comps.n2, *n2));
        }
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = SimpleObject::glass_sphere();
        *shape.transform_mut() = Matrix4::translation(0., 0., 1.);

        let i = Intersection::new(&TorUVT::JustT { t: 5. }, shape);
        let xs = [i];
        let comps = i.prepare_computations(r, &xs);

        assert!(comps.under_point.z > EPSILON / 2.);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = SimpleObject::glass_sphere();
        let r = Ray::new(
            Tuple::point(0., 0., 2_f64.sqrt() / 2.),
            Tuple::vector(0., 1., 0.),
        );
        let xs = vec![
            Intersection::new(
                &TorUVT::JustT {
                    t: -2_f64.sqrt() / 2.,
                },
                shape,
            ),
            Intersection::new(
                &TorUVT::JustT {
                    t: 2_f64.sqrt() / 2.,
                },
                shape,
            ),
        ];
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();

        assert!(approx_equal(reflectance, 1.));
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = SimpleObject::glass_sphere();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));
        let xs = vec![
            Intersection::new(&TorUVT::JustT { t: -1. }, shape),
            Intersection::new(&TorUVT::JustT { t: 1. }, shape),
        ];
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();

        assert!(approx_equal(reflectance, 0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = SimpleObject::glass_sphere();
        let r = Ray::new(Tuple::point(0., 0.99, -2.), Tuple::vector(0., 0., 1.));
        let xs = vec![Intersection::new(&TorUVT::JustT { t: 1.8589 }, shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        let reflectance = comps.schlick();

        assert!(approx_equal(reflectance, 0.48873));
    }
}
