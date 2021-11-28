use crate::color::Color;
use crate::fuzzy_eq::*;
use std::vec::Vec;

pub trait Sized {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn canvas(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.get_pixel_index(x, y)]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = color;
    }

    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[cfg(test)]
mod tests_canvas {
    use crate::assert_fuzzy_eq;
    use super::*;

    #[test]
    fn create_canvas() {
        let c: Canvas = Canvas::canvas(10, 20);

        assert_eq!(10, c.width);
        assert_eq!(20, c.height);

        for x in 0..c.width {
            for y in 0..c.height {
                assert_fuzzy_eq!(c.pixel_at(x, y), Color::black())
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut canvas = Canvas::canvas(10, 20);

        let red = Color::color(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);

        let expected_color = Color::color(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(expected_color, canvas.pixel_at(2, 3));
    }
}

