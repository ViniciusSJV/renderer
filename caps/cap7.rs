extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use renderer::camera::Camera;
use renderer::color::Color;
use renderer::lights::Light;
use renderer::matrix::Matrix;
use renderer::object::Object;
use renderer::sphere::Sphere;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

fn main() {
    let mut floor = Sphere::default();
    floor.transform = Matrix::scaling(Tuple::vector(10., 0.01, 10.));
    floor.material.color = Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = Sphere::default();
    left_wall.transform = Matrix::translation(Tuple::vector(0., 0., 5.)) *
        Matrix::rotation_y(-PI/4.) * Matrix::rotation_x(PI/2.) *
        Matrix::scaling(Tuple::vector(10., 0.01, 10.));
    left_wall.material = floor.material;

    let mut right_wall = Sphere::default();
    right_wall.transform = Matrix::translation(Tuple::vector(0., 0., 5.)) *
        Matrix::rotation_y(PI/4.) * Matrix::rotation_x(PI/2.) *
        Matrix::scaling(Tuple::vector(10., 0.01, 10.));
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.transform = Matrix::translation(Tuple::vector(-0.5, 1., 0.5));
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.specular = 0.3;
    middle.material.diffuse = 0.7;

    let mut right = Sphere::default();
    right.transform = Matrix::translation(Tuple::vector(1.5, 0.5, -0.5)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1., 0.1);
    right.material.specular = 0.3;
    right.material.diffuse = 0.7;

    let mut left = Sphere::default();
    left.transform = Matrix::translation(Tuple::vector(-1.5, 0.33, -0.75)) * Matrix::scaling(Tuple::vector(0.33, 0.33, 0.33));
    left.material.color = Color::new(1., 0.8, 0.1);
    left.material.specular = 0.3;
    left.material.diffuse = 0.7;

    let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let s1 = Object::from(floor);
    let s2 = Object::from(left_wall);
    let s3 = Object::from(right_wall);
    let s4 = Object::from(middle);
    let s5 = Object::from(right);
    let s6 = Object::from(left);

    let world = World::new(vec![s1, s2, s3, s4, s5, s6], vec![light]);

    let from = Tuple::point(0., 1.5, -5.);
    let to  = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(1000, 500, PI/3.).with_transform(
        from.view_transform(to, up)
    );

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();
    write("./cap8.ppm", ppm).expect("Error.")
}