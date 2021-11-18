use crate::{color::Color, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub enum Pattern {
    Striped(StripePattern),
}

impl Pattern {
    pub fn striped(a: Color, b: Color) -> Self {
        Self::Striped(StripePattern::new(a, b))
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        match self {
            Self::Striped(pat) => pat.stripe_at(point),
        }
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

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.stripe_at(Tuple::point(0., 1., 0.)), Color::white());
        assert_eq!(pattern.stripe_at(Tuple::point(0., 2., 0.)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 1.)), Color::white());
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 2.)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(pattern.stripe_at(Tuple::point(0.9, 0., 0.)), Color::white());
        assert_eq!(pattern.stripe_at(Tuple::point(1., 0., 0.)), Color::black());
        assert_eq!(
            pattern.stripe_at(Tuple::point(-0.1, 0., 0.)),
            Color::black()
        );
        assert_eq!(pattern.stripe_at(Tuple::point(-1., 0., 0.)), Color::black());
        assert_eq!(
            pattern.stripe_at(Tuple::point(-1.1, 0., 0.)),
            Color::white()
        );
    }
}
