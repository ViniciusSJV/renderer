extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use renderer::camera::Camera;
use renderer::color::Color;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::Matrix;
use renderer::object::{Intersectable, Object};
use renderer::patterns::{Checkers, Patterns};
use renderer::plane::Plane;
use renderer::sphere::Sphere;
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

    let mut a = Sphere::grass(1.);
    a.material.specular = 0.1;
    a.material.diffuse = 0.8;
    a.material.shininess = 300.;
    a.transform = Matrix::translation(Tuple::vector(0., 1., -0.5));

    let mut b = Sphere::default();
    b.material.specular = 0.3;
    b.material.diffuse = 0.7;
    b.material.color = Color::new(0., 1., 0.);
    b.transform = Matrix::translation(Tuple::vector(0.33, 0.5, 4.5)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5));

    let mut c = Sphere::default();
    c.material.specular = 0.3;
    c.material.diffuse = 0.7;
    c.material.color = Color::new(0., 0., 1.);
    c.transform = Matrix::translation(Tuple::vector(-1.5, 0.5, 3.)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5));

    let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let obj1 = Object::from(a);
    let obj2 = Object::from(b);
    let obj3 = Object::from(c);
    let obj4 = Object::from(wall);
    let obj5 = Object::from(floor);

    let world = World::new(vec![obj1, obj2, obj3, obj4, obj5], vec![light]);

    let from = Tuple::point(0., 1.5, -5.);
    let to  = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(2048, 1080, PI/3.).with_transform(
        from.view_transform(to, up)
    );

    let canvas = camera.render(world);

    let png = canvas.to_png();
    write("./cap11-final-fresnel-effect.png", png).expect("Error.")
}