use crate::misc::EPSILON;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn local_intersect(local_ray: Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -local_ray.origin.y / local_ray.direction.y;

            vec![t]
        }
    }

    pub fn local_normal_at(_: Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::SimpleObject;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let n1 = Plane::local_normal_at(Tuple::point(0., 0., 0.));
        let n2 = Plane::local_normal_at(Tuple::point(10., 0., -10.));
        let n3 = Plane::local_normal_at(Tuple::point(-5., 0., 150.));

        assert_eq!(n1, Tuple::vector(0., 1., 0.));
        assert_eq!(n2, Tuple::vector(0., 1., 0.));
        assert_eq!(n3, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));
        let xs = Plane::local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let xs = Plane::local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = SimpleObject::plane();
        let local_ray = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));
        let world_ray = local_ray.transform(p.transform());
        let xs = p.intersect(world_ray);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, p);
    }
    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = SimpleObject::plane();
        let local_ray = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));
        let world_ray = local_ray.transform(p.transform());
        let xs = p.intersect(world_ray);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, p);
    }
}
