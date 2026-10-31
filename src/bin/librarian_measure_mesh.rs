//! Medicao para o Librarian: dois teapots, um de vidro (com Fresnel) e um espelho ao lado,
//! cada um numa BVH, sobre o chao xadrez e diante de uma parede. Imprime os tempos no
//! formato `librarian-measure nome=valor unidade` e grava a imagem, cujo hash o Librarian
//! guarda para conferir que nenhum pixel mudou.
//! Uso: librarian_measure_mesh <saida.png> <modelo.obj> [largura] [altura]
extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use std::time::Instant;
use renderer::camera::Camera;
use renderer::color::Color;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::Matrix;
use renderer::obj::load_bvh;
use renderer::object::{Intersectable, Object};
use renderer::patterns::{Checkers, Patterns};
use renderer::plane::Plane;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let usage = "Uso: librarian_measure_mesh <saida.png> <modelo.obj> [largura] [altura]";
    let output = args.get(1).expect(usage);
    let model_path = args.get(2).expect(usage);
    let width = args.get(3).map_or(64, |w| w.parse().expect("largura"));
    let height = args.get(4).map_or(64, |h| h.parse().expect("altura"));

    let mut glass = Material::glass();
    glass.color = Color::new(0.82, 0.95, 1.0);
    glass.ambient = 0.05;
    glass.diffuse = 0.1;
    glass.specular = 0.9;
    glass.shininess = 300.;

    let mut mirror = Material::phong();
    mirror.color = Color::new(0.1, 0.1, 0.1);
    mirror.ambient = 0.;
    mirror.diffuse = 0.1;
    mirror.specular = 1.;
    mirror.shininess = 300.;
    mirror.reflective = 0.9;

    let scale = Matrix::scaling(Tuple::vector(0.4, 0.4, 0.4));
    let start = Instant::now();
    let glass_teapot = load_bvh(
        model_path,
        glass,
        Matrix::translation(Tuple::vector(-1.5, 0., 0.)) * Matrix::rotation_y(PI / 6.) * scale,
    );
    let mirror_teapot = load_bvh(
        model_path,
        mirror,
        Matrix::translation(Tuple::vector(1.5, 0., 0.)) * Matrix::rotation_y(-PI / 6.) * scale,
    );
    let load_ms = start.elapsed().as_secs_f64() * 1000.0;

    let mut checkers = Material::phong();
    checkers.pattern = Some(Patterns::from(Checkers::default()));
    checkers.specular = 0.;

    let mut floor = Plane::default();
    floor.set_material(checkers);

    let mut wall = Plane::default();
    wall.set_transform(Matrix::translation(Tuple::vector(0., 0., 6.)) * Matrix::rotation_x(PI / 2.));
    wall.set_material(checkers);

    let lights = vec![Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.))];
    let mut world = World::new(vec![Object::from(floor), Object::from(wall)], lights);
    world.groups.push(glass_teapot);
    world.groups.push(mirror_teapot);

    let from = Tuple::point(0., 2.5, -6.);
    let to = Tuple::point(0., 0.6, 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(width, height, PI / 3.).with_transform(from.view_transform(to, up));

    renderer::count::reset();
    let start = Instant::now();
    let canvas = camera.render(world);
    let render_ms = start.elapsed().as_secs_f64() * 1000.0;

    write(output, canvas.to_png()).expect("Nao foi possivel gravar a imagem");
    println!("librarian-measure load_ms={load_ms:.3} ms");
    println!("librarian-measure render_ms={render_ms:.3} ms");
    println!("librarian-measure pixels={} px", width * height);
    // Com a feature librarian-count, quantas vezes cada linha contada rodou no render.
    renderer::count::report();
}
