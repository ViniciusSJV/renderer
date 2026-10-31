//! Medicao para o Librarian: renderiza uma malha OBJ (o teapot) numa BVH sobre o chao
//! xadrez, imprime os tempos no formato `librarian-measure nome=valor unidade` e grava a
//! imagem, cujo hash o Librarian guarda para conferir que nenhum pixel mudou.
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

    let mut material = Material::phong();
    material.color = Color::new(0.9, 0.3, 0.2);
    material.specular = 0.4;
    material.shininess = 100.;

    let start = Instant::now();
    let model = load_bvh(
        model_path,
        material,
        Matrix::rotation_y(PI / 6.) * Matrix::scaling(Tuple::vector(0.6, 0.6, 0.6)),
    );
    let load_ms = start.elapsed().as_secs_f64() * 1000.0;

    let mut floor_material = Material::phong();
    floor_material.pattern = Some(Patterns::from(Checkers::default()));
    let mut floor = Plane::default();
    floor.set_material(floor_material);

    let lights = vec![Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.))];
    let mut world = World::new(vec![Object::from(floor)], lights);
    world.groups.push(model);

    let from = Tuple::point(0., 3., -6.);
    let to = Tuple::point(0., 0.8, 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(width, height, PI / 3.).with_transform(from.view_transform(to, up));

    let start = Instant::now();
    let canvas = camera.render(world);
    let render_ms = start.elapsed().as_secs_f64() * 1000.0;

    write(output, canvas.to_png()).expect("Nao foi possivel gravar a imagem");
    println!("librarian-measure load_ms={load_ms:.3} ms");
    println!("librarian-measure render_ms={render_ms:.3} ms");
    println!("librarian-measure pixels={} px", width * height);
}
