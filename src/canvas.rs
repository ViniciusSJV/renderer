use crate::color::Color;
use crate::fuzzy_eq::*;
use std::vec::Vec;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
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

    fn create_ppm_header(&self) -> Vec<u8> {
        let mut header = Vec::new();
        header.extend(String::from("P3\n").into_bytes());
        header.extend(format!("{} {}\n", self.width, self.height).into_bytes());
        header.extend(format!("{}", 255).into_bytes());

        return header;
    }

    fn to_ppm(&self) -> Vec<u8> {
        let header = self.create_ppm_header();
        return header;
    }
}

#[cfg(test)]
mod tests_canvas {
    use crate::assert_fuzzy_eq;
    use super::*;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(10, canvas.width);
        assert_eq!(20, canvas.height);

        for x in 0..canvas.width {
            for y in 0..canvas.height {
                assert_fuzzy_eq!(canvas.pixel_at(x, y), Color::black())
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut canvas = Canvas::new(10, 20);

        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);

        let expected_color = Color::new(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(expected_color, canvas.pixel_at(2, 3));
    }

    #[test]
    fn constructing_the_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm_image = canvas.to_ppm();
        /*
         * Magic number: P3
         * Width/Height: 5 3
         * Maximum Color Value: 255
         */
        let expected_result = String::from("P3\n5 3\n255").into_bytes();

        assert_eq!(ppm_image, expected_result);
    }
}

