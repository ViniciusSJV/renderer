extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use renderer::canvas::Canvas;
use renderer::color::Color;
use renderer::matrix::Matrix;
use renderer::object::Intersectable;
use renderer::ray::Ray;
use renderer::sphere::Sphere;
use renderer::tuple::Tuple;

fn main() {
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixel = 500;
    let pixel_size = wall_size / (canvas_pixel as f64);
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixel, canvas_pixel);
    let red = Color::new(1., 0., 0.);
    let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
    let ray_origin = Tuple::point(0., 0., -5.);

    for y in 0..canvas_pixel {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixel {
            let world_x = -half + pixel_size * (x as f64);

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            sphere.set_transform(Matrix::rotation_z(PI / 4.) * Matrix::scaling(Tuple::vector(0.5, 1., 1.)));

            let xs = sphere.intersect(ray);

            if xs.hit() != None {
                canvas.set_pixel_color(x, y, red);
            }
        }
    }

    let ppm = canvas.to_ppm();
    write("./cap5.ppm", ppm).expect("Error.")
}