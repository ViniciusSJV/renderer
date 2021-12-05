use std::fs::write;

mod fuzzy_eq;
mod tuple;
mod color;
mod canvas;
mod matrix;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::tuple::Tuple;

fn main() {
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