use crate::{intersection::Intersection, ray::Ray};

use super::{Object, SimpleObject};

#[derive(Clone, PartialEq, Debug)]
pub struct Csg {
    op: CsgOp,
    pub(crate) left: Box<Object>,
    pub(crate) right: Box<Object>,
}

impl Csg {
    fn new(op: CsgOp, left: Object, right: Object) -> Self {
        Self {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn union(left: Object, right: Object) -> Self {
        Self::new(CsgOp::Union, left, right)
    }

    pub(crate) fn intersection(left: Object, right: Object) -> Self {
        Self::new(CsgOp::Intersection, left, right)
    }

    pub(crate) fn difference(left: Object, right: Object) -> Self {
        Self::new(CsgOp::Difference, left, right)
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        let left_intersections = self.left.intersect(local_ray);
        let right_intersections = self.right.intersect(local_ray);

        let mut xs = left_intersections
            .into_iter()
            .chain(right_intersections.into_iter())
            .collect::<Vec<_>>();
        xs.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());

        self.filter_intersections(xs)
    }

    #[allow(dead_code)]
    pub(crate) fn filter_intersections<'a>(
        &self,
        intersections: Vec<Intersection<'a>>,
    ) -> Vec<Intersection<'a>> {
        let mut inl = false;
        let mut inr = false;
        let mut result = vec![];

        for i in intersections {
            let left_hit = self.left.includes(i.object);

            if self.op.intersection_allowed(left_hit, inl, inr) {
                result.push(i);
            }

            if left_hit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }

        result
    }

    pub(crate) fn includes(&self, object: SimpleObject) -> bool {
        self.left.includes(object) || self.right.includes(object)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum CsgOp {
    Union,
    Intersection,
    Difference,
}

impl CsgOp {
    fn intersection_allowed(&self, left_hit: bool, inl: bool, inr: bool) -> bool {
        match self {
            CsgOp::Union => (left_hit && !inr) || (!left_hit && !inl),
            CsgOp::Intersection => (left_hit && inr) || (!left_hit && inl),
            CsgOp::Difference => (left_hit && !inr) || (!left_hit && inl),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::{matrix4::Matrix4, tuple::Tuple},
        misc::approx_equal,
        shape::SimpleObject,
    };

    use super::*;

    #[test]
    fn csg_is_created_with_an_operation_and_two_shapes() {
        let s1 = Object::sphere();
        let s2 = Object::cube();
        let c = Csg::union(s1.clone(), s2.clone());

        let s11: &Object = &c.left;
        assert_eq!(s11, &s1);
    }

    #[rustfmt::skip]
    #[test]
    fn evaluating_the_rules_for_csg_operations() {
        assert_eq!(CsgOp::Union.intersection_allowed(true, true, true),  false);
        assert_eq!(CsgOp::Union.intersection_allowed(true, true, false),  true);
        assert_eq!(CsgOp::Union.intersection_allowed(true, false, true),  false);
        assert_eq!(CsgOp::Union.intersection_allowed(true, false, false),  true);
        assert_eq!(CsgOp::Union.intersection_allowed(false, true, true),  false);
        assert_eq!(CsgOp::Union.intersection_allowed(false, true, false),  false);
        assert_eq!(CsgOp::Union.intersection_allowed(false, false, true),  true);
        assert_eq!(CsgOp::Union.intersection_allowed(false, false, false),  true);
        assert_eq!(CsgOp::Intersection.intersection_allowed(true, true, true),  true);
        assert_eq!(CsgOp::Intersection.intersection_allowed(true, true, false),  false);
        assert_eq!(CsgOp::Intersection.intersection_allowed(true, false, true),  true);
        assert_eq!(CsgOp::Intersection.intersection_allowed(true, false, false),  false);
        assert_eq!(CsgOp::Intersection.intersection_allowed(false, true, true),  true);
        assert_eq!(CsgOp::Intersection.intersection_allowed(false, true, false),  true);
        assert_eq!(CsgOp::Intersection.intersection_allowed(false, false, true),  false);
        assert_eq!(CsgOp::Intersection.intersection_allowed(false, false, false),  false);
        assert_eq!(CsgOp::Difference.intersection_allowed(true, true, true),  false);
        assert_eq!(CsgOp::Difference.intersection_allowed(true, true, false),  true);
        assert_eq!(CsgOp::Difference.intersection_allowed(true, false, true),  false);
        assert_eq!(CsgOp::Difference.intersection_allowed(true, false, false),  true);
        assert_eq!(CsgOp::Difference.intersection_allowed(false, true, true),  true);
        assert_eq!(CsgOp::Difference.intersection_allowed(false, true, false),  true);
        assert_eq!(CsgOp::Difference.intersection_allowed(false, false, true),  false);
        assert_eq!(CsgOp::Difference.intersection_allowed(false, false, false),  false);
    }

    #[test]
    fn filtering_a_list_of_intersections() {
        let s1 = Object::sphere();
        let s2 = Object::cube();
        let examples = vec![
            (CsgOp::Union, 0, 3),
            (CsgOp::Intersection, 1, 2),
            (CsgOp::Difference, 0, 1),
        ];

        let shape1 = SimpleObject::from_object(&s1).unwrap();
        let shape2 = SimpleObject::from_object(&s2).unwrap();

        for (op, x0, x1) in examples {
            let c = Csg::new(op, s1.clone(), s2.clone());
            let xs = vec![
                Intersection::new_(1., shape1),
                Intersection::new_(2., shape2),
                Intersection::new_(3., shape1),
                Intersection::new_(4., shape2),
            ];
            let result = c.filter_intersections(xs.clone());

            assert_eq!(result.len(), 2);
            assert_eq!(result[0], xs[x0]);
            assert_eq!(result[1], xs[x1]);
        }
    }

    #[test]
    fn a_ray_misses_a_csg_object() {
        let c = Object::union(Object::sphere(), Object::cube());
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let xs = c.intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_hits_a_csg_object() {
        let s1 = Object::sphere();
        let mut s2 = Object::sphere();
        s2.transform = Matrix4::translation(0., 0., 0.5);
        let c = Object::union(s1.clone(), s2.clone());
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = c.intersect(r);

        assert_eq!(xs.len(), 2);

        assert!(approx_equal(xs[0].t, 4.));
        assert_eq!(xs[0].object, SimpleObject::from_object(&s1).unwrap());
        assert!(approx_equal(xs[1].t, 6.5));
        assert_eq!(xs[1].object, SimpleObject::from_object(&s2).unwrap());
    }
}
