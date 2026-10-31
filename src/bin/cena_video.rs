//! Cena do video: dois teapots suaves (vidro com Fresnel e espelho) e a Suzanne dourada entre
//! eles, sobre o chao xadrez, diante de uma parede xadrez, com bolinhas coloridas em volta. As bolinhas sao sorteadas com semente
//! fixa (a imagem sai sempre igual) e descartadas quando encostam num teapot, noutra bolinha ou
//! ficam fora da camera.
//! Uso: cena_video <saida.png> [largura] [altura]
extern crate renderer;

use std::f64::consts::PI;
use std::fs::write;
use std::time::Instant;
use renderer::camera::Camera;
use renderer::color::Color;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::Matrix;
use renderer::obj::load_bvh_smooth;
use renderer::object::{Intersectable, Object};
use renderer::patterns::{Checkers, Patterns};
use renderer::plane::Plane;
use renderer::sphere::Sphere;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

/// Gerador simples e deterministico (LCG), so para sortear as bolinhas.
struct Sorteio(u64);

impl Sorteio {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn between(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
}

/// Uma cor viva a partir do matiz (0..1).
fn hue(h: f64) -> Color {
    let channel = |offset: f64| {
        let k = (h * 6. + offset) % 6.;
        1. - (k.min(4. - k).clamp(0., 1.))
    };
    Color::new(channel(5.) * 0.85 + 0.1, channel(3.) * 0.85 + 0.1, channel(1.) * 0.85 + 0.1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output = args.get(1).expect("Uso: cena_video <saida.png> [largura] [altura]");
    let width = args.get(2).map_or(2048, |w| w.parse().expect("largura"));
    let height = args.get(3).map_or(1080, |h| h.parse().expect("altura"));

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
    let teapots = [(-1.85, PI / 6.), (1.85, -PI / 6.)];
    let glass_teapot = load_bvh_smooth("teapot.obj", glass,
        Matrix::translation(Tuple::vector(teapots[0].0, 0., 0.)) * Matrix::rotation_y(teapots[0].1) * scale);
    let mirror_teapot = load_bvh_smooth("teapot.obj", mirror,
        Matrix::translation(Tuple::vector(teapots[1].0, 0., 0.)) * Matrix::rotation_y(teapots[1].1) * scale);

    // A Suzanne do arquivo fica fora da origem: centro em (-2.494, _, 4.104), base em y 0.267.
    let mut gold = Material::phong();
    gold.color = Color::new(1.0, 0.72, 0.25);
    gold.diffuse = 0.6;
    gold.specular = 1.;
    gold.shininess = 250.;
    gold.reflective = 0.3;
    let suzanne_at = (0., 0.7);
    let suzanne = load_bvh_smooth("suzanne.obj", gold,
        Matrix::translation(Tuple::vector(suzanne_at.0, 0., suzanne_at.1)) * Matrix::rotation_y(PI) * Matrix::scaling(Tuple::vector(0.45, 0.45, 0.45))
            * Matrix::translation(Tuple::vector(2.494, -0.267, -4.104)));
    // O que as bolinhas nao podem tocar nem esconder: (x, z, raio no chao).
    let obstacles = [(teapots[0].0, 0., 1.6), (teapots[1].0, 0., 1.6), (suzanne_at.0, suzanne_at.1, 0.75)];

    let mut checkers = Material::phong();
    checkers.pattern = Some(Patterns::from(Checkers::new(Color::new(0.85, 0.85, 0.85), Color::new(0.06, 0.06, 0.08))));
    checkers.specular = 0.;
    checkers.reflective = 0.08;

    let mut floor = Plane::default();
    floor.set_material(checkers);
    let mut wall = Plane::default();
    wall.set_transform(Matrix::translation(Tuple::vector(0., 0., 6.)) * Matrix::rotation_x(PI / 2.));
    let mut wall_material = checkers;
    wall_material.reflective = 0.;
    wall.set_material(wall_material);

    // Bolinhas: raio, posicao no chao, longe dos teapots e umas das outras, dentro da camera.
    let mut sorteio = Sorteio(2026);
    let mut balls: Vec<(f64, f64, f64)> = vec![];
    let mut tries = 0;
    while balls.len() < 8 && tries < 10_000 {
        tries += 1;
        let r = sorteio.between(0.15, 0.32);
        let z = sorteio.between(-1.8, 2.5);
        let x = sorteio.between(-1., 1.) * 0.4 * (z + 7.);
        let clear_of_teapots = obstacles.iter().all(|(ox, oz, or)| ((x - ox).powi(2) + (z - oz).powi(2)).sqrt() > or + r);
        // Nada atras de um teapot, visto da camera.
        // Vista da camera, nenhuma bolinha encosta num objeto: ou fica bem na frente (abaixo dele na
        // imagem), ou separada na horizontal.
        let visible = obstacles.iter().all(|(ox, oz, or)| z < oz - 1.4 || (x / (z + 7.) - ox / (oz + 7.)).abs() > (r / (z + 7.) + or / (oz + 7.)) * 1.15);
        let clear_of_balls = balls.iter().all(|(bx, bz, br)| ((x - bx).powi(2) + (z - bz).powi(2)).sqrt() > r + br + 0.08);
        // E separadas vistas da camera tambem: uma nao pode ficar na frente da outra na imagem.
        let apart_on_screen = balls.iter().all(|(bx, bz, br)| (x / (z + 7.) - bx / (bz + 7.)).abs() > (r / (z + 7.) + br / (bz + 7.)) * 1.2);
        if clear_of_teapots && clear_of_balls && apart_on_screen && visible {
            balls.push((x, z, r));
        }
    }
    let mut objects = vec![Object::from(floor), Object::from(wall)];
    for (i, (x, z, r)) in balls.iter().enumerate() {
        let mut ball = Sphere::default();
        ball.transform = Matrix::translation(Tuple::vector(*x, *r, *z)) * Matrix::scaling(Tuple::vector(*r, *r, *r));
        // Matizes espalhados pela proporcao aurea: cores diferentes umas das outras.
        ball.material.color = hue((0.05 + i as f64 * 0.618_033_988_75) % 1.);
        ball.material.diffuse = 0.7;
        ball.material.specular = 0.6;
        ball.material.shininess = 200.;
        ball.material.reflective = if i % 4 == 0 { 0.35 } else { 0.1 };
        objects.push(Object::from(ball));
    }

    let lights = vec![
        Light::point_light(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.)),
        Light::point_light(Tuple::point(8., 6., -8.), Color::new(0.35, 0.35, 0.4)),
    ];
    let mut world = World::new(objects, lights);
    world.groups.push(glass_teapot);
    world.groups.push(mirror_teapot);
    world.groups.push(suzanne);

    let from = Tuple::point(0., 2.6, -7.);
    let to = Tuple::point(0., 0.6, 0.);
    let up = Tuple::vector(0., 1., 0.);
    let camera = Camera::new(width, height, PI / 3.).with_transform(from.view_transform(to, up));

    let start = Instant::now();
    let canvas = camera.render(world);
    let render_ms = start.elapsed().as_secs_f64() * 1000.0;
    write(output, canvas.to_png()).expect("Nao foi possivel gravar a imagem");
    println!("bolinhas: {}", balls.len());
    println!("librarian-measure render_ms={render_ms:.3} ms");
}
