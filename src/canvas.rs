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

    pub fn set_pixel_color(&mut self, x: usize, y: usize, color: Color) {
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
        header.extend(format!("{}\n", 255).into_bytes());

        return header;
    }

    //https://github.com/lopossumi/Rust-Output-Image
    fn create_ppm_pixel_data(&self) -> Vec<u8> {
        let mut pixel_data: Vec<u8> = Vec::new();
        let mut i = 1;
        for pixel in self.pixels.iter() {
            let color = pixel.clamp(0.0, 1.0);

            let red: u8 = (color.red * 255.).round() as u8;
            let green: u8 = (color.green * 255.).round() as u8;
            let blue: u8 = (color.blue * 255.).round() as u8;

            pixel_data.extend(format!("{} {} {}", red, green, blue).into_bytes());
            if i % 5 == 0 {
                pixel_data.extend(String::from("\n").into_bytes());
            } else {
                pixel_data.extend(String::from(" ").into_bytes());
            }
            i += 1;
        }
        pixel_data
    }

    pub fn to_ppm(&self) -> Vec<u8> {
        let header = self.create_ppm_header();
        let pixel_data = self.create_ppm_pixel_data();

        let mut ppm = Vec::new();
        ppm.extend(header);
        ppm.extend(pixel_data);
        ppm
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
        canvas.set_pixel_color(2, 3, red);

        let expected_color = Color::new(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(expected_color, canvas.pixel_at(2, 3));
    }

    #[test]
    fn constructing_the_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        /*
         * Magic number: P3
         * Width/Height: 5 3
         * Maximum Color Value: 255
         */
        let expected_header = String::from("P3\n5 3\n255\n").into_bytes();

        assert_eq!(&ppm[..11], expected_header);
    }

    #[test]
    fn set_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let color_1 = Color::new(1.5, 0., 0.);
        let color_2 = Color::new(0.0, 0.5, 0.);
        let color_3 = Color::new(-0.5, 0., 1.);

        canvas.set_pixel_color(0,0, color_1);
        canvas.set_pixel_color(2,1, color_2);
        canvas.set_pixel_color(4,2, color_3);

        let result = canvas.to_ppm();

        let header = String::from("P3\n5 3\n255\n").into_bytes();
        let data = String::from("\
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        ).into_bytes();

        let mut expected_result = Vec::new();
        expected_result.extend(header);
        expected_result.extend(data);

        assert_eq!(expected_result, result);
    }
}

