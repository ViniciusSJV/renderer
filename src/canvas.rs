use crate::color::Color;
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

    pub fn get_pixel_color(&self, x: usize, y: usize) -> Color {
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

    fn create_ppm_pixel_data(&self) -> Vec<u8> {
        let mut pixel_data: Vec<u8> = Vec::new();

        let mut total_caracteres_in_line: u8 = 0;
        let limit_caracteres_in_line: u8 = 69;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.get_pixel_color(x, y);
                let clamp_color = pixel.clamp(0.0, 1.0);

                let red: u8 = (clamp_color.red * 255.).round() as u8;
                let green: u8 = (clamp_color.green * 255.).round() as u8;
                let blue: u8 = (clamp_color.blue * 255.).round() as u8;

                let mut data;

                for (i, color) in [red, green, blue].iter().enumerate() {
                    data = format!("{}", color);

                    total_caracteres_in_line += data.chars().count() as u8;
                    if total_caracteres_in_line + 2 >= limit_caracteres_in_line {
                        //hit the line character limit? add a line break
                        data = format!("{}\n", data);
                        total_caracteres_in_line = 0;
                    } else {
                        let is_last_column: bool = x == self.width - 1;
                        if is_last_column && i == 2 {
                            //if is last column in line? add a line break
                            data = format!("{}\n", data);
                            total_caracteres_in_line = 0;
                        } else {
                            data = format!("{} ", data);
                            total_caracteres_in_line += 1;
                        }
                    }
                    pixel_data.extend(data.into_bytes());
                }
            }
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
    use crate::assert_equivalent;
    use crate::equivalent::*;
    use super::*;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(10, canvas.width);
        assert_eq!(20, canvas.height);

        for x in 0..canvas.width {
            for y in 0..canvas.height {
                assert_equivalent!(canvas.get_pixel_color(x, y), Color::black())
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut canvas = Canvas::new(10, 20);

        let red = Color::new(1.0, 0.0, 0.0);
        canvas.set_pixel_color(2, 3, red);

        let expected_color = Color::new(1.0, 0.0, 0.0);

        assert_equivalent!(expected_color, canvas.get_pixel_color(2, 3));
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

    #[test]
    fn splitting_long_lines_in_ppm() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);

        for x in 0..canvas.width {
            for y in 0..canvas.height {
                canvas.set_pixel_color(x,y, color);
            }
        }

        let result = canvas.to_ppm();

        let header = String::from("P3\n10 2\n255\n").into_bytes();
        let data = String::from("\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        ).into_bytes();

        let mut expected_result = Vec::new();
        expected_result.extend(header);
        expected_result.extend(data);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn ppm_files_terminated_by_newline() {
        let canvas = Canvas::new(5, 3);

        let _ppm = canvas.to_ppm();

        let expected_result = String::from("\n").into_bytes();

        assert_eq!(expected_result, expected_result);
    }
}

