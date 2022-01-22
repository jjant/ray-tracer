use crate::{color::Color, math::matrix4::Matrix4, math::tuple::Tuple, shape::SimpleObject};

#[derive(Clone, Copy, Debug)]
pub struct Pattern {
    transform: Matrix4,
    pattern_type: PatternType,
}

#[derive(Clone, Copy, Debug)]
enum PatternType {
    Striped(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checkered(CheckeredPattern),
    #[cfg(test)]
    TestPattern,
}

impl Pattern {
    fn new(pattern_type: PatternType) -> Self {
        Self {
            transform: Matrix4::identity(),
            pattern_type,
        }
    }

    pub fn transform_mut(&mut self) -> &mut Matrix4 {
        &mut self.transform
    }

    pub fn striped(a: Color, b: Color) -> Self {
        Self::new(PatternType::Striped(StripePattern::new(a, b)))
    }

    #[allow(dead_code)]
    pub fn gradient(a: Color, b: Color) -> Self {
        Self::new(PatternType::Gradient(GradientPattern::new(a, b)))
    }

    #[allow(dead_code)]
    pub fn ring(a: Color, b: Color) -> Self {
        Self::new(PatternType::Ring(RingPattern::new(a, b)))
    }

    pub fn checkered(a: Color, b: Color) -> Self {
        Self::new(PatternType::Checkered(CheckeredPattern::new(a, b)))
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        match self.pattern_type {
            PatternType::Striped(pattern_type) => pattern_type.pattern_at(point),
            PatternType::Gradient(pattern_type) => pattern_type.pattern_at(point),
            PatternType::Ring(pattern_type) => pattern_type.pattern_at(point),
            PatternType::Checkered(pattern_type) => pattern_type.pattern_at(point),
            #[cfg(test)]
            PatternType::TestPattern => tests::TestPattern::pattern_at(point),
        }
    }

    pub fn pattern_at_object(&self, object: SimpleObject, world_point: Tuple) -> Color {
        let object_point = object.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.pattern_at(pattern_point)
    }
}
#[derive(Clone, Copy, Debug)]
struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct GradientPattern {
    a: Color,
    b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        let t = point.x - point.x.floor();

        self.a + (self.b - self.a) * t
    }
}

#[derive(Debug, Clone, Copy)]
struct RingPattern {
    a: Color,
    b: Color,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        let p = (point.x.powi(2) + point.z.powi(2)).floor() as i32 % 2 == 0;

        if p {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CheckeredPattern {
    a: Color,
    b: Color,
}

impl CheckeredPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        let sum_floors = point.x.floor() + point.y.floor() + point.z.floor();
        let predicate = sum_floors as i32 % 2 == 0;

        if predicate {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::SimpleObject;

    impl Pattern {
        pub fn test() -> Self {
            Self::new(PatternType::TestPattern)
        }
    }

    #[derive(Copy, Clone)]
    pub struct TestPattern {}
    impl TestPattern {
        pub fn pattern_at(point: Tuple) -> Color {
            Color::new(point.x, point.y, point.z)
        }
    }

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 1., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 2., 0.)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 1.)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 2.)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.9, 0., 0.)),
            Color::white()
        );
        assert_eq!(pattern.pattern_at(Tuple::point(1., 0., 0.)), Color::black());
        assert_eq!(
            pattern.pattern_at(Tuple::point(-0.1, 0., 0.)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-1., 0., 0.)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-1.1, 0., 0.)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = SimpleObject::sphere();
        *object.transform_mut() = Matrix4::scaling(2., 2., 2.);

        let pattern = Pattern::striped(Color::white(), Color::black());
        let c = pattern.pattern_at_object(object, Tuple::point(1.5, 0., 0.));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = SimpleObject::sphere();
        let mut pattern = Pattern::striped(Color::white(), Color::black());
        *pattern.transform_mut() = Matrix4::scaling(2., 2., 2.);

        let c = pattern.pattern_at_object(object, Tuple::point(1.5, 0., 0.));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = SimpleObject::sphere();
        *object.transform_mut() = Matrix4::scaling(2., 2., 2.);

        let mut pattern = Pattern::striped(Color::white(), Color::black());
        *pattern.transform_mut() = Matrix4::translation(0.5, 0., 0.);

        let c = pattern.pattern_at_object(object, Tuple::point(2.5, 0., 0.));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = Pattern::test();

        assert_eq!(pattern.transform, Matrix4::identity())
    }

    #[test]
    fn assigning_a_transformation() {
        let mut pattern = Pattern::test();

        *pattern.transform_mut() = Matrix4::translation(1., 2., 3.);

        assert_eq!(pattern.transform, Matrix4::translation(1., 2., 3.));
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let mut shape = SimpleObject::sphere();
        *shape.transform_mut() = Matrix4::scaling(2., 2., 2.);
        let pattern = Pattern::test();
        let c = pattern.pattern_at_object(shape, Tuple::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let shape = SimpleObject::sphere();
        let mut pattern = Pattern::test();
        *pattern.transform_mut() = Matrix4::scaling(2., 2., 2.);
        let c = pattern.pattern_at_object(shape, Tuple::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut shape = SimpleObject::sphere();
        *shape.transform_mut() = Matrix4::scaling(2., 2., 2.);
        let mut pattern = Pattern::test();
        *pattern.transform_mut() = Matrix4::translation(0.5, 1., 1.5);

        let c = pattern.pattern_at_object(shape, Tuple::point(2.5, 3., 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::gradient(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::ring(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(1., 0., 0.)), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 1.)), Color::black());
        // 0.708 = just slightly more than √2/2
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.708, 0., 0.708)),
            Color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::checkered(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.99, 0., 0.)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(1.01, 0., 0.)),
            Color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::checkered(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0.99, 0.)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 1.01, 0.)),
            Color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::checkered(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.99)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 1.01)),
            Color::black()
        );
    }
}
