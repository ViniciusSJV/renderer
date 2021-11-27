use crate::tuple::Tuple;

mod fuzzy_eq;
mod tuple;

fn main() {
    let enviroment = World::world(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0)
    );

    let projectile = Object::object(
        Tuple::vector(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.0, 0.0)
    );

    let mut current_projectile = projectile;
    let mut i:i32 = 0;
    while current_projectile.position.y > 0.0 {
        println!("{}: {:?}", i, current_projectile);
        current_projectile = tick(&enviroment, &current_projectile);
        i += 1;
    }
    println!("{}: {:?}", i, current_projectile);
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