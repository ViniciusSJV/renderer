use std::f64::consts::{PI, TAU};
use std::fs::write;

use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;

mod equivalent;
mod tuple;
mod color;
mod canvas;
mod matrix;
mod ray;
mod sphere;
mod intersection;
mod object;
mod lights;
mod materials;
mod world;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::lights::Light;
use crate::matrix::{Matrix};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

fn main() {
    //cap1_cap2();
    //cap3();
    //cap4();
    //cap5();
    cap6();
}

fn cap6() {
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixel = 500;
    let pixel_size = wall_size / (canvas_pixel as f64);
    let half = wall_size / 2.;

    let canvas_mutex = Mutex::new(Canvas::new(canvas_pixel, canvas_pixel));

    let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
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

            let point = hit.ray.position(hit.t);
            let normal = hit.object.normal_at(point);
            let eye = -hit.ray.direction;

            let color = hit.object.material().lighting(light, point, eye, normal);

            let mut canvas = canvas_mutex.lock().unwrap();
            canvas.set_pixel_color(x, y, color);
        }
    });

    let canvas = canvas_mutex.lock().unwrap();
    let ppm = canvas.to_ppm();
    write("./result-4.ppm", ppm).expect("Error.")
}

fn cap5() {
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixel = 500;
    let pixel_size = wall_size / (canvas_pixel as f64);
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixel, canvas_pixel);
    let red = Color::new(1., 0., 0.);
    let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
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
    write("./result-3.ppm", ppm).expect("Error.")
}

fn cap4() {
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
    write("./result-2.ppm", ppm).expect("Error.")
}

fn cap3() {
    let mat4_identity: Matrix<4> = Matrix::identity();

    println!("Mat4 identity: {:?}", mat4_identity);
    println!("Mat4 identity inverse: {:?}", mat4_identity.inverse());

    let mat4: Matrix<4> = Matrix::from([
        [-6., 1., 1., 6.],
        [-8., 5., 8., 6.],
        [-1., 0., 8., 2.],
        [-7., 1., -1., 1.]
    ]);

    println!("Mat4 original: {:?}", mat4);
    println!("Mat4 mult by inverse: {:?}", mat4 * mat4.inverse());
}

fn cap1_cap2() {
    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1., 1., 0.);

    let enviroment = World::world(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0)
    );

    let projectile = ObjectWorld::object(
        Tuple::vector(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25
    );

    let mut current_projectile = projectile;
    while current_projectile.position.y > 0.0 {
        let x = current_projectile.position.x.round() as usize;
        let y = canvas.height - current_projectile.position.y.round() as usize;
        canvas.set_pixel_color(x, y, color);
        current_projectile = tick(&enviroment, &current_projectile);
    }

    let ppm = canvas.to_ppm();
    write("./result.ppm", ppm).expect("Error.")
}

pub struct World {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
pub struct ObjectWorld {
    position: Tuple,
    velocity: Tuple,
}

impl ObjectWorld {
    pub fn object(position: Tuple, velocity: Tuple) -> Self{
        ObjectWorld{ position, velocity }
    }
}

impl World {
    pub fn world(gravity: Tuple, wind: Tuple) -> Self{
        World{ gravity, wind }
    }
}

pub fn tick (world: &World, object: &ObjectWorld) -> ObjectWorld{
    ObjectWorld::object(
        object.position + object.velocity,
        object.velocity + world.gravity + world.wind
    )
}