extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use renderer::camera::Camera;
use renderer::color::Color;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::Matrix;
use renderer::object::{Intersectable, Object};
use renderer::patterns::{Checkers, Incuse, LinearGradient, Patterns, Ring, Stripe};
use renderer::plane::Plane;
use renderer::sphere::Sphere;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

fn main() {
    let mut material = Material::phong();
    material.specular = 0.;
    //let mut pattern1 = Ring::new(Color::new(1., 0., 0., ), Color::new(0., 1., 0.));
    //material.pattern = Option::from(Patterns::from(pattern1));

    let mut pattern1 = Checkers::new(Color::new(1., 0., 0., ), Color::new(0., 1., 0.));
    pattern1.set_pattern_transform(Matrix::translation(Tuple::vector(-0.5, 1., 0.5)));
    material.pattern = Option::from(Patterns::from(pattern1));

    let mut floor = Plane::default();
    floor.set_material(material);

    let mut middle = Sphere::default();
    middle.transform = Matrix::translation(Tuple::vector(-0.5, 1., 0.5));
    middle.material.specular = 0.3;
    middle.material.diffuse = 0.7;
    let mut pattern2 = Ring::new(Color::new(0.2, 0.8, 0.6, ), Color::new(0., 1., 0.));
    pattern2.set_pattern_transform(Matrix::translation(Tuple::vector(2.5, 2.5, 2.5)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5)));
    middle.material.pattern = Option::from(Patterns::from(pattern2));


    let mut right = Sphere::default();
    right.transform = Matrix::translation(Tuple::vector(1.5, 0.5, -0.5)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5));
    right.material.specular = 0.3;
    right.material.diffuse = 0.7;
    let mut pattern3 = Stripe::new(Color::new(0., 0., 1.0, ), Color::new(1., 0.2, 1.));
    pattern3.set_pattern_transform(right.transform);
    right.material.pattern = Option::from(Patterns::from(pattern3));

    let mut left = Sphere::default();
    left.transform = Matrix::translation(Tuple::vector(-1.5, 0.33, -0.75)) * Matrix::scaling(Tuple::vector(0.33, 0.33, 0.33));
    left.material.specular = 0.3;
    left.material.diffuse = 0.7;
    let pattern4 = LinearGradient::new(Color::new(0.06, 0.1, 0.5), Color::white());
    left.material.pattern = Option::from(Patterns::from(pattern4));

    let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let obj1 = Object::from(middle);
    let obj2 = Object::from(right);
    let obj3 = Object::from(left);

    let obj4 = Object::from(floor);

    let world = World::new(vec![obj1, obj2, obj3, obj4], vec![light]);

    let from = Tuple::point(0., 1.5, -5.);
    let to  = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(1000, 500, PI/3.).with_transform(
        from.view_transform(to, up)
    );

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();
    write("./cap10.ppm", ppm).expect("Error.")
}