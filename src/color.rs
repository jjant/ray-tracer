use crate::misc::approx_equal;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self {
            red: 0.,
            green: 0.,
            blue: 0.,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    }

    #[allow(dead_code)]
    pub fn red() -> Self {
        Self {
            red: 1.,
            green: 0.,
            blue: 0.,
        }
    }

    #[allow(dead_code)]
    pub fn green() -> Self {
        Self {
            red: 0.,
            green: 1.,
            blue: 0.,
        }
    }

    #[allow(dead_code)]
    pub fn blue() -> Self {
        Self {
            red: 0.,
            green: 0.,
            blue: 1.,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let d_red = self.red - other.red;
        let d_green = self.green - other.green;
        let d_blue = self.blue - other.blue;
        let dist_squared = d_red.powi(2) + d_green.powi(2) + d_blue.powi(2);

        approx_equal(dist_squared, 0.)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert!(approx_equal(c.red, -0.5));
        assert!(approx_equal(c.green, 0.4));
        assert!(approx_equal(c.blue, 1.7));
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn colors_using_rgb255() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);
        let red = Color::new(1., 0., 0.);
        let green = Color::new(0., 1., 0.);
        let blue = Color::new(0., 0., 1.);

        assert_eq!(white, Color::white());
        assert_eq!(black, Color::black());
        assert_eq!(red, Color::red());
        assert_eq!(green, Color::green());
        assert_eq!(blue, Color::blue());
    }
}
