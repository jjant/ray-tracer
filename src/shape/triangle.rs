use crate::{math::tuple::Tuple, misc::EPSILON, ray::Ray, shape::BoundingBox};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub(crate) p1: Tuple,
    pub(crate) p2: Tuple,
    pub(crate) p3: Tuple,
    kind: TriangleKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TriangleKind {
    Flat,
    Smooth { n1: Tuple, n2: Tuple, n3: Tuple },
}

impl Triangle {
    pub(crate) fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        Self {
            p1,
            p2,
            p3,
            kind: TriangleKind::Flat,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn smooth(p1: Tuple, p2: Tuple, p3: Tuple, n1: Tuple, n2: Tuple, n3: Tuple) -> Self {
        Self {
            p1,
            p2,
            p3,
            kind: TriangleKind::Smooth { n1, n2, n3 },
        }
    }

    fn edge1(&self) -> Tuple {
        self.p2 - self.p1
    }

    fn edge2(&self) -> Tuple {
        self.p3 - self.p1
    }

    fn normal(&self) -> Tuple {
        self.edge2().cross(self.edge1()).normalize()
    }

    pub(crate) fn local_normal_at(&self, uvt: &UVT) -> Tuple {
        let UVT { u, v, .. } = uvt;

        match self.kind {
            TriangleKind::Flat => self.normal(),
            TriangleKind::Smooth { n1, n2, n3 } => {
                (n2 * *u + n3 * *v + n1 * (1. - *u - *v)).normalize()
            }
        }
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<UVT> {
        let dir_cross_edge2 = local_ray.direction.cross(self.edge2());
        let det = self.edge1().dot(dir_cross_edge2);

        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = local_ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_edge2);
        if u < 0. || u > 1. {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(self.edge1());
        let v = f * local_ray.direction.dot(origin_cross_e1);
        if v < 0. || (u + v) > 1. {
            return vec![];
        }

        let t = f * self.edge2().dot(origin_cross_e1);
        vec![UVT { u, v, t }]
    }

    pub(crate) fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(&[self.p1, self.p2, self.p3])
    }
}

#[derive(Clone, Copy)]
pub(crate) struct UVT {
    pub(crate) t: f64,
    pub(crate) u: f64,
    pub(crate) v: f64,
}

#[cfg(test)]
mod tests {
    use crate::{
        intersection::{Intersection, TorUVT},
        misc::approx_equal,
        shape::{Object, Shape, SimpleObject},
    };

    use super::*;

    impl Triangle {
        pub(crate) fn normals(&self) -> (Tuple, Tuple, Tuple) {
            match self.kind {
                TriangleKind::Flat => (self.normal(), self.normal(), self.normal()),
                TriangleKind::Smooth { n1, n2, n3 } => (n1, n2, n3),
            }
        }
    }

    fn test_smooth_tri() -> Triangle {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);
        let n1 = Tuple::vector(0., 1., 0.);
        let n2 = Tuple::vector(-1., 0., 0.);
        let n3 = Tuple::vector(1., 0., 0.);

        Triangle::smooth(p1, p2, p3, n1, n2, n3)
    }

    #[test]
    fn constructing_a_triangle() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);
        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.edge1(), Tuple::vector(-1., -1., 0.));
        assert_eq!(t.edge2(), Tuple::vector(1., -1., 0.));
        assert_eq!(t.normal(), Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let uvt1 = UVT {
            t: 0.,
            u: 0.5,
            v: 0.25,
        };
        let uvt2 = UVT {
            t: 0.,
            u: 0.75,
            v: 0.25,
        };
        let uvt3 = UVT {
            t: 0.,
            u: 0.25,
            v: 0.5,
        };
        let n1 = t.local_normal_at(&uvt1);
        let n2 = t.local_normal_at(&uvt2);
        let n3 = t.local_normal_at(&uvt3);

        assert_eq!(n1, t.normal());
        assert_eq!(n2, t.normal());
        assert_eq!(n3, t.normal());
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 1., 0.));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1p3_edge() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(1., 1., -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1p2_edge() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(-1., 1., -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p2p3_edge() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(0., 0.5, -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert!(approx_equal(xs[0].t, 2.));
    }

    #[test]
    fn a_smooth_triangle_uses_uv_to_interpolate_the_normal() {
        let i = UVT {
            u: 0.45,
            v: 0.25,
            t: 1.,
        };
        let tri = test_smooth_tri();
        let n = tri.local_normal_at(&i);

        assert_eq!(n, Tuple::vector(-0.5547, 0.83205, 0.));
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let uvt = UVT {
            t: 1.,
            u: 0.45,
            v: 0.25,
        };
        let r = Ray::new(Tuple::point(-0.2, 0.3, -2.), Tuple::vector(0., 0., 1.));
        let tri = test_smooth_tri();
        let shape = Shape::Triangle(tri);
        let object = Object::new(shape);
        let shape = SimpleObject::from_object(&object).unwrap();
        let i = Intersection::new(&TorUVT::UVT { uvt }, shape);
        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(comps.normal_vector, Tuple::vector(-0.5547, 0.83205, 0.));
    }
}
