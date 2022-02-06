extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use renderer::camera::Camera;
use renderer::color::Color;
use renderer::cube::Cube;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::Matrix;
use renderer::object::{Intersectable, Object};
use renderer::patterns::{Checkers, Incuse, Patterns, Ring};
use renderer::plane::Plane;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

fn main() {
    let mut material = Material::phong();
    let pattern1 = Checkers::default();
    material.pattern = Option::from(Patterns::from(pattern1));

    let mut floor = Plane::default();
    floor.set_material(material);

    let mut wall = Plane::default();
    wall.set_transform(Matrix::translation(Tuple::vector(0., 0., 10.)) * Matrix::rotation_x(PI/2.));
    wall.set_material(material);

    let mut mirror = Cube::default();
    mirror.material.specular = 0.0;
    mirror.material.diffuse = 0.0;
    mirror.material.reflective = 1.;
    mirror.transform = Matrix::translation(Tuple::vector(0., 0., 9.5)) * Matrix::scaling(Tuple::vector(5.0, 3.5, 0.1));

    let mut b = Cube::default();
    b.material.specular = 0.3;
    b.material.diffuse = 0.7;
    b.material.color = Color::new(0., 1., 0.);
    b.transform = Matrix::translation(Tuple::vector(1., 0.5, -1.5)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5));

    let mut c = Cube::default();
    c.material.specular = 0.3;
    c.material.diffuse = 0.7;
    let mut ring = Ring::new(Color::new(0.6, 0., 0.4), Color::new(0., 0.5, 0.2));
    ring.set_pattern_transform(Matrix::scaling(Tuple::vector(0.1, 0.1, 0.1)));
    c.material.pattern = Option::from(Patterns::from(ring));
    c.transform = Matrix::translation(Tuple::vector(2.5, 0.5, 6.)) * Matrix::scaling(Tuple::vector(0.5, 1.5, 0.5));

    let mut d = Cube::default();
    d.material.specular = 0.2;
    d.material.diffuse = 0.5;
    d.material.transparency = 0.5;
    d.material.reflective = 0.3;
    d.material.reflactive_index = 1.5;
    d.transform = Matrix::translation(Tuple::vector(-1.6, 1.0, 1.4)) * Matrix::scaling(Tuple::vector(1.0, 1.0, 1.0)) * Matrix::rotation_y(PI/1.5);

    let light1 = Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
    let light2 = Light::point_light(Tuple::point(10., 8., 0.), Color::new(1., 1., 1.));

    let obj1 = Object::from(mirror);
    let obj2 = Object::from(b);
    let obj3 = Object::from(c);
    let obj4 = Object::from(wall);
    let obj5 = Object::from(floor);
    let obj6 = Object::from(d);

    let world = World::new(vec![obj1, obj2, obj3, obj4, obj5, obj6], vec![light1, light2]);

    let from = Tuple::point(0., 1.5, -5.);
    let to  = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(2048, 1080, PI/3.).with_transform(
        from.view_transform(to, up)
    );

    let canvas = camera.render(world);

    let png = canvas.to_png();
    write("./cap12.png", png).expect("Error.")
}