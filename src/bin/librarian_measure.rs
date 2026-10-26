//! Medicao para o Librarian: renderiza a cena do capitulo 11 com duas luzes, imprime o tempo
//! no formato `librarian-measure nome=valor unidade` e grava a imagem, cujo hash o Librarian
//! guarda para conferir que nenhum pixel mudou.
//! Uso: librarian_measure <saida.png> [largura] [altura]
extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use std::time::Instant;
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
    let args: Vec<String> = std::env::args().collect();
    let output = args.get(1).expect("Uso: librarian_measure <saida.png> [largura] [altura]");
    let width = args.get(2).map_or(640, |w| w.parse().expect("largura"));
    let height = args.get(3).map_or(338, |h| h.parse().expect("altura"));

    let mut material = Material::phong();
    material.pattern = Option::from(Patterns::from(Checkers::default()));

    let mut floor = Plane::default();
    floor.set_material(material);

    let mut wall = Plane::default();
    wall.set_transform(Matrix::translation(Tuple::vector(0., 0., 10.)) * Matrix::rotation_x(PI / 2.));
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

    let lights = vec![
        Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.)),
        Light::point_light(Tuple::point(10., 10., -10.), Color::new(0.4, 0.4, 0.4)),
    ];
    let objects = vec![Object::from(a), Object::from(b), Object::from(c), Object::from(wall), Object::from(floor)];
    let world = World::new(objects, lights);

    let from = Tuple::point(0., 1.5, -5.);
    let to = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(width, height, PI / 3.).with_transform(from.view_transform(to, up));

    let start = Instant::now();
    let canvas = camera.render(world);
    let render_ms = start.elapsed().as_secs_f64() * 1000.0;

    write(output, canvas.to_png()).expect("Nao foi possivel gravar a imagem");
    println!("librarian-measure render_ms={render_ms:.3} ms");
    println!("librarian-measure pixels={} px", width * height);
}
