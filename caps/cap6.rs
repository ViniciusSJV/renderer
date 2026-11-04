extern crate renderer;

use std::fs::write;
use std::sync::Mutex;
use itertools::Itertools;
use rayon::prelude::*;
use renderer::canvas::Canvas;
use renderer::color::Color;
use renderer::lights::Light;
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

    let canvas_mutex = Mutex::new(Canvas::new(canvas_pixel, canvas_pixel));

    let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
    sphere.material.color = Color::new(1., 0.2, 1.);
    let ray_origin = Tuple::point(0., 0., -5.);

    let light = Light::point_light(
        Tuple::point(-10., 10., -10.),
        Color::white()
    );

    (0..canvas_pixel)
        .cartesian_product(0..canvas_pixel)
        .par_bridge()
        .for_each(|(x, y)| {
            let world_x = -half + pixel_size * (x as f64);
            let world_y = half - pixel_size * (y as f64);

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            let xs = sphere.intersect(ray);

            if xs.hit() != None {
                let hit = xs.hit().unwrap();

                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;

                let color = hit.object.material().lighting(hit.object, light, point, eye, normal, true);

                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.set_pixel_color(x, y, color);
            }
        });

    let canvas = canvas_mutex.lock().unwrap();
    let ppm = canvas.to_ppm();
    write("./cap6.ppm", ppm).expect("Error.")
}