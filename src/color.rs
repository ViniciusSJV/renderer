use std::ops;
use crate::fuzzy_eq::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub(crate) red: f64,
    pub(crate) green: f64,
    pub(crate) blue: f64
}

impl Color {
    pub fn color(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn black() -> Self {
        Color::color(0.0, 0.0, 0.0)
    }

    pub fn clamp(&self, lower_bound: f64, upper_bound: f64) -> Color {
        Color::color(
            self.red.min(upper_bound).max(lower_bound),
            self.green.min(upper_bound).max(lower_bound),
            self.blue.min(upper_bound).max(lower_bound),
        )
    }
}

impl FuzzyEq<Color> for Color {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.red.fuzzy_eq(other.red)
            && self.green.fuzzy_eq(other.green)
            && self.blue.fuzzy_eq(other.blue)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color::color(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue
        )
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color::color(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue
        )
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Color::color(
            self.red * other,
            self.green * other,
            self.blue * other
        )
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color::color(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue
        )
    }
}

#[cfg(test)]
mod tests_color {
    use crate::assert_fuzzy_eq;
    use super::*;

    #[test]
    fn color_does_fill_properties() {
        let point = Color::color(-0.5, 0.4, 1.7);

        assert_fuzzy_eq!(point.red, -0.5);
        assert_fuzzy_eq!(point.green, 0.4);
        assert_fuzzy_eq!(point.blue, 1.7);
    }

    #[test]
    fn add_to_colors() {
        let color_1 = Color::color(0.9,0.6,0.75);
        let color_2 = Color::color(0.7,0.1,0.25);

        let color_expected = Color::color(1.6,0.7,1.0);

        assert_fuzzy_eq!(color_1 + color_2, color_expected);
    }

    #[test]
    fn sub_tow_colors() {
        let color_1 = Color::color(0.9,0.6,0.75);
        let color_2 = Color::color(0.7,0.1,0.25);

        let color_expected = Color::color(0.2,0.5,0.5);

        assert_fuzzy_eq!(color_1 - color_2, color_expected);
    }

    #[test]
    fn mul_color_by_scalar() {
        let color = Color::color(0.2,0.3,0.4);
        let scarlar = 2.;
        let color_expected = Color::color(0.4,0.6,0.8);

        assert_fuzzy_eq!(color * scarlar, color_expected);
    }

    #[test]
    fn mul_color() {
        let color_1 = Color::color(1.0,0.2,0.4);
        let color_2 = Color::color(0.9,1.0,0.1);

        let color_expected = Color::color(0.9,0.2,0.04);

        assert_fuzzy_eq!(color_1 * color_2, color_expected);
    }

    #[test]
    fn clamping_colors() {
        let color = Color::color(2.3, -6.7, 0.8);

        let expected_result = Color::color(1.0, 0.0, 0.8);
        let actual_result = color.clamp(0.0, 1.0);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

}