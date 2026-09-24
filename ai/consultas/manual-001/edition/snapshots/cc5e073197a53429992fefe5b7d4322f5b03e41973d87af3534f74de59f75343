extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use renderer::camera::Camera;
use renderer::color::Color;
use renderer::cone::Cone;
use renderer::cylinder::Cylinder;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::Matrix;
use renderer::object::{Intersectable, Object};
use renderer::patterns::{Checkers, Patterns, Stripe};
use renderer::plane::Plane;
use renderer::sphere::Sphere;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

fn main() {

    let mut floor = Plane::default();
    let mut floor_material = Material::phong();
    let checkers = Checkers::default();
    floor_material.pattern = Option::from(Patterns::from(checkers));
    floor.set_material(floor_material);

    let mut wall = Plane::default();
    wall.set_transform(Matrix::translation(Tuple::vector(0., 0., 10.)) * Matrix::rotation_x(PI/2.));
    let mut wall_material = Material::phong();
    let stripe = Stripe::default();
    wall_material.pattern = Option::from(Patterns::from(stripe));
    wall.set_material(wall_material);

    let light1 = Light::point_light(Tuple::point(-10., 10., -5.), Color::new(1., 1., 1.));
    let light2 = Light::point_light(Tuple::point(10., 8., 0.), Color::new(0.5, 0.5, 0.5));

    let obj1 = Object::from(wall);
    let obj2 = Object::from(floor);

    let mut cone = Cone::default();
    cone.maximum = 1.;
    cone.minimum = 0.;
    cone.closed = false;
    cone.material.color = Color::new(0.66, 0.28, 0.);
    cone.set_transform(
        Matrix::scaling(Tuple::point(0.25, 0.5, 0.5)) *
            Matrix::translation(Tuple::point(0., 1.0, 0.))
    );

    let mut b1 = Sphere::default();
    b1.transform = Matrix::translation(Tuple::vector(0., 1., 0.)) * Matrix::scaling(Tuple::vector(0.22, 0.22, 0.22));
    b1.material.specular = 0.1;
    b1.material.diffuse = 1.0;
    b1.material.color = Color::new(0.5, 0., 0.);

    let mut b2 = Sphere::default();
    b2.transform = Matrix::translation(Tuple::vector(0., 1.3, 0.)) * Matrix::scaling(Tuple::vector(0.22, 0.22, 0.22));
    b2.material.specular = 0.1;
    b2.material.diffuse = 1.0;
    b2.material.color = Color::new(0.0, 0.5, 0.);

    let mut b3 = Sphere::default();
    b3.transform = Matrix::translation(Tuple::vector(0., 1.6, 0.)) * Matrix::scaling(Tuple::vector(0.22, 0.22, 0.22));
    b3.material.specular = 0.1;
    b3.material.diffuse = 1.0;
    b3.material.color = Color::new(0.0, 0., 0.5);


    let mut cyl1 = Cylinder::default();
    cyl1.maximum = 0.8;
    cyl1.minimum = 0.0;
    cyl1.closed = false;
    cyl1.material.color = Color::new(150.0, 75.0, 0.);
    cyl1.set_transform(
        Matrix::scaling(Tuple::point(1.5, 0.33, 1.5)) *
            Matrix::translation(Tuple::point(0., 0., 0.))
    );

    let mut cyl2 = Cylinder::default();
    cyl2.maximum = 0.8;
    cyl2.minimum = 0.0;
    cyl2.closed = false;
    cyl2.material.color = Color::new(0.3, 0.0, 0.8);
    cyl2.set_transform(
        Matrix::scaling(Tuple::point(1., 0.33, 1.0)) *
            Matrix::translation(Tuple::point(0., 0., 0.))
    );

    let mut cyl3 = Cylinder::default();
    cyl3.maximum = 1.4;
    cyl3.minimum = 0.0;
    cyl3.closed = true;
    cyl3.material.color = Color::new(0.0, 1.0, 0.2);
    cyl3.set_transform(
        Matrix::scaling(Tuple::point(0.5, 0.33, 0.5)) *
            Matrix::translation(Tuple::point(0., 0., 0.))
    );

    let mut cyl4 = Cylinder::default();
    cyl4.maximum = 2.0;
    cyl4.minimum = 0.0;
    cyl4.closed = true;
    cyl4.material.reflective = 1.;
    cyl4.set_transform(
        Matrix::translation(Tuple::point(-2., 0., 4.3))
    );

    let mut cyl5 = Cylinder::default();
    cyl5.maximum = 1.6;
    cyl5.minimum = 0.0;
    cyl5.closed = false;
    cyl5.material.diffuse = 0.;
    cyl5.material.specular = 0.;
    cyl5.material.reflective = 0.5;
    cyl5.material.transparency = 0.8;
    cyl5.set_transform(
        Matrix::translation(Tuple::point(2., 0., 4.3))
    );

    let mut cyl6 = Cylinder::default();
    cyl6.maximum = 2.5;
    cyl6.minimum = 0.0;
    cyl6.closed = true;
    cyl6.material.color = Color::new(0.7, 0.0, 0.0);
    cyl6.set_transform(
        Matrix::translation(Tuple::point(2., 0., 4.3)) *
            Matrix::scaling(Tuple::point(0.3, 1.0, 0.3))
    );

    let mut cone2 = Cone::default();
    cone2.maximum = 2.;
    cone2.minimum = -2.;
    cone2.closed = false;
    cone2.material.color = Color::new(0.5, 0.5, 0.5);
    cone2.set_transform(
            Matrix::translation(Tuple::point(-2.5, 0.4, 1.)) *
                Matrix::scaling(Tuple::point(0.2, 0.2, 0.2)) *
            Matrix::rotation_z(PI/2.)
    );

    let obj3 = Object::from(cone);
    let obj4 = Object::from(b1);
    let obj5 = Object::from(b2);
    let obj6 = Object::from(b3);

    let obj7 = Object::from(cyl1);
    let obj8 = Object::from(cyl2);
    let obj9 = Object::from(cyl3);

    let obj10 = Object::from(cyl4);

    let obj11 = Object::from(cyl5);
    let obj12 = Object::from(cyl6);

    let obj13 = Object::from(cone2);

    let world = World::new(vec![obj1, obj2, obj3, obj4, obj5, obj6, obj7, obj8, obj9, obj10, obj11, obj12, obj13], vec![light1, light2]);

    let from = Tuple::point(0., 1.8, -5.);
    let to  = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(2048, 1800, PI/3.).with_transform(
        from.view_transform(to, up)
    );

    let canvas = camera.render(world);

    let png = canvas.to_png();
    write("./cap13.png", png).expect("Error.")
}