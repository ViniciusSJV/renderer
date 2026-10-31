extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;

use renderer::camera::Camera;
use renderer::color::Color;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::Matrix;
use renderer::obj::load_bvh;
use renderer::object::Object;
use renderer::patterns::{Checkers, Patterns};
use renderer::plane::Plane;
use renderer::object::Intersectable;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

fn render_suzanne(material: Material, output: &str) {
    let model = load_bvh(
        "suzanne.obj",
        material,
        Matrix::translation(Tuple::vector(2.5, -0.27, -4.1)),
    );
    let mut floor = Plane::default();
    let mut floor_material = Material::phong();
    floor_material.pattern = Some(Patterns::from(Checkers::default()));
    floor.set_material(floor_material);

    let objects = vec![Object::from(floor)];

    let light = Light::point_light(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );
    let mut world = World::new(objects, vec![light]);
    world.groups.push(model);
    let camera = Camera::new(1920, 1080, PI / 3.)
        .with_maximum_recursive_depth(2)
        .with_transform(
        Tuple::point(0., 1.5, -5.).view_transform(
            Tuple::point(0., 1., 0.),
            Tuple::vector(0., 1., 0.),
        ),
    );

    write(output, camera.render(world).to_png()).expect("Error writing render");
}

fn main() {
    let mut material = Material::glass();
    material.color = Color::new(0.82, 0.95, 1.0);
    material.ambient = 0.05;
    material.diffuse = 0.8;
    material.specular = 0.1;
    material.shininess = 300.;
    render_suzanne(material, "./cap15.png");
}
