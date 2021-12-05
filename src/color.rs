use std::ops;
use crate::equivalent::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub(crate) red: f64,
    pub(crate) green: f64,
    pub(crate) blue: f64
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn clamp(&self, lower_bound: f64, upper_bound: f64) -> Color {
        Color::new(
            self.red.min(upper_bound).max(lower_bound),
            self.green.min(upper_bound).max(lower_bound),
            self.blue.min(upper_bound).max(lower_bound),
        )
    }
}

impl Equivalence<Color> for Color {
    fn equivalent(&self, other: Self) -> bool {
        self.red.equivalent(other.red)
            && self.green.equivalent(other.green)
            && self.blue.equivalent(other.blue)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue
        )
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue
        )
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Color::new(
            self.red * other,
            self.green * other,
            self.blue * other
        )
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue
        )
    }
}

#[cfg(test)]
mod tests_color {
    use crate::assert_equivalent;
    use super::*;

    #[test]
    fn color_does_fill_properties() {
        let point = Color::new(-0.5, 0.4, 1.7);

        assert_equivalent!(point.red, -0.5);
        assert_equivalent!(point.green, 0.4);
        assert_equivalent!(point.blue, 1.7);
    }

    #[test]
    fn add_to_colors() {
        let color_1 = Color::new(0.9, 0.6, 0.75);
        let color_2 = Color::new(0.7, 0.1, 0.25);

        let color_expected = Color::new(1.6, 0.7, 1.0);

        assert_equivalent!(color_1 + color_2, color_expected);
    }

    #[test]
    fn sub_tow_colors() {
        let color_1 = Color::new(0.9, 0.6, 0.75);
        let color_2 = Color::new(0.7, 0.1, 0.25);

        let color_expected = Color::new(0.2, 0.5, 0.5);

        assert_equivalent!(color_1 - color_2, color_expected);
    }

    #[test]
    fn mul_color_by_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        let scarlar = 2.;
        let color_expected = Color::new(0.4, 0.6, 0.8);

        assert_equivalent!(color * scarlar, color_expected);
    }

    #[test]
    fn mul_color() {
        let color_1 = Color::new(1.0, 0.2, 0.4);
        let color_2 = Color::new(0.9, 1.0, 0.1);

        let color_expected = Color::new(0.9, 0.2, 0.04);

        assert_equivalent!(color_1 * color_2, color_expected);
    }

    #[test]
    fn clamping_colors() {
        let color = Color::new(2.3, -6.7, 0.8);

        let expected_result = Color::new(1.0, 0.0, 0.8);
        let actual_result = color.clamp(0.0, 1.0);

        assert_equivalent!(actual_result, expected_result);
    }

}