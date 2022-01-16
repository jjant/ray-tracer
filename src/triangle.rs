use crate::{misc::EPSILON, ray::Ray, shape::BoundingBox, tuple::Tuple};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
}

impl Triangle {
    pub(crate) fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        Self { p1, p2, p3 }
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

    pub(crate) fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        self.normal()
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<f64> {
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
        vec![t]
    }

    pub(crate) fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(&[self.p1, self.p2, self.p3])
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::approx_equal;

    use super::*;

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
        let n1 = t.local_normal_at(Tuple::point(0., 0.5, 0.));
        let n2 = t.local_normal_at(Tuple::point(-0.5, 0.75, 0.));
        let n3 = t.local_normal_at(Tuple::point(0.5, 0.25, 0.));

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
        assert!(approx_equal(xs[0], 2.));
    }
}
