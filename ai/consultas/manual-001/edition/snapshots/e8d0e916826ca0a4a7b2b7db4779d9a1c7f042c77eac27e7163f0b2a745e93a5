extern crate renderer;

use std::f64::consts::{TAU};
use std::fs::write;
use renderer::canvas::Canvas;
use renderer::color::Color;
use renderer::matrix::Matrix;
use renderer::tuple::Tuple;

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let color_white = Color::white();

    let origin = Tuple::point(1000./2., 1000./2., 0.);
    let transf_origin = Matrix::translation(origin);

    for h in 0..12 {
        let r = 300.;
        let transf_rotate_z = Matrix::rotation_z(TAU / 12. * (h as f64));
        let point = Tuple::point(0., r, 0.);

        let point_rotate = transf_origin * transf_rotate_z * point;

        let x = point_rotate.x.round() as usize;
        let y = canvas.height - point_rotate.y.round() as usize;
        canvas.set_pixel_color(x, y, color_white);
    }

    let ppm = canvas.to_ppm();
    write("./cap4.ppm", ppm).expect("Error.")
}