use std::fs::write;

mod equivalent;
mod tuple;
mod color;
mod canvas;
mod matrix;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::Matrix4;
use crate::tuple::Tuple;

fn main() {
    cap1_cap2();
    cap3();
}

fn cap3() {
    let mat4_identity: Matrix4 = Matrix4::identity();

    println!("Mat4 identity: {:?}", mat4_identity);
    println!("Mat4 identity inverse: {:?}", mat4_identity.inverse());

    let mat4: Matrix4 = Matrix4::new([
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