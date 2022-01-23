use super::Object;

#[derive(Clone, PartialEq, Debug)]
pub struct CSG {
    op: CsgOp,
    left: Box<Object>,
    right: Box<Object>,
}

impl CSG {
    pub(crate) fn union(left: Object, right: Object) -> Self {
        Self {
            op: CsgOp::Union,
            left: Box::new(left),
            right: Box::new(right),
        }
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
    use super::*;

    #[test]
    fn csg_is_created_with_an_operation_and_two_shapes() {
        let s1 = Object::sphere();
        let s2 = Object::cube();
        let c = CSG::union(s1.clone(), s2.clone());

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
}
