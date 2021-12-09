use std::f64::consts::PI;
use std::fs::write;

mod equivalent;
mod tuple;
mod color;
mod canvas;
mod matrix;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::{Matrix};
use crate::tuple::Tuple;

fn main() {
    cap1_cap2();
    cap3();
    cap4();
}

fn cap4() {
    let mut canvas = Canvas::new(1000, 1000);
    let color_white = Color::new(1., 1., 1.);

    let origin = Tuple::point(1000./2., 1000./2., 0.);
    let transf_origin = Matrix::translation(origin);

    for h in 0..12 {
        let r = 300.;
        let transf_rotate_z = Matrix::rotation_z(2. * PI / 12. * (h as f64));
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

    let projectile = Object::object(
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
pub struct Object {
    position: Tuple,
    velocity: Tuple,
}

impl Object {
    pub fn object(position: Tuple, velocity: Tuple) -> Self{
        Object{ position, velocity }
    }
}

impl World {
    pub fn world(gravity: Tuple, wind: Tuple) -> Self{
        World{ gravity, wind }
    }
}

pub fn tick (world: &World, object: &Object) -> Object{
    Object::object(
        object.position + object.velocity,
        object.velocity + world.gravity + world.wind
    )
}