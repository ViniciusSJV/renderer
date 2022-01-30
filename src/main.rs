use std::f64::consts::{PI, TAU};
use std::fs::write;

use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;
use renderer::camera::Camera;

use renderer::canvas::Canvas;
use renderer::color::Color;
use renderer::lights::Light;
use renderer::materials::Material;
use renderer::matrix::{Matrix};
use renderer::object::{Intersectable, Object};
use renderer::patterns::{Checkers, Incuse, LinearGradient, Patterns, Ring, Stripe};
use renderer::plane::Plane;
use renderer::sphere::Sphere;
use renderer::ray::Ray;
use renderer::transformations::Transform;
use renderer::tuple::Tuple;
use renderer::world::World;

fn main() {
    //cap1_cap2();
    //cap3();
    //cap4();
    //cap5();
    //cap6();
    //cap7();
    //cap9();
    //cap10();
    cap11();
}

fn cap11() {
    let mut material = Material::phong();
    material.specular = 0.;
    let mut pattern1 = Checkers::default();
    material.pattern = Option::from(Patterns::from(pattern1));

    let mut floor = Plane::default();
    floor.set_material(material);

    let mut wall = Plane::default();
    wall.set_transform(Matrix::translation(Tuple::vector(0., 0., 10.)) * Matrix::rotation_x(PI/2.));
    wall.set_material(material);

    let mut a = Sphere::grass();
    a.material.specular = 0.;
    a.material.diffuse = 0.;
    a.transform = Matrix::translation(Tuple::vector(0., 1., -0.5));

    let mut b = Sphere::default();
    b.material.reflective = 1.;
    b.material.specular = 0.3;
    b.material.diffuse = 0.7;
    b.material.color = Color::new(0., 1., 0.);
    b.transform = Matrix::translation(Tuple::vector(0.33, 1., 4.5)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5));

    let mut c = Sphere::default();
    c.material.specular = 0.3;
    c.material.diffuse = 0.7;
    c.material.color = Color::new(0., 0., 1.);
    c.transform = Matrix::translation(Tuple::vector(-1.5, 1., 3.)) * Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5));

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
    write("./cap11-refraction.png", png).expect("Error.")
}

fn cap10() {
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

fn cap9() {
    let mut material = Material::phong();
    material.specular = 0.;
    material.color = Color::new(0.5, 0.5 ,0.5);

    let mut floor = Plane::new(Tuple::point(0., 0., 0.));
    floor.set_material(material);

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
    write("./cap9.ppm", ppm).expect("Error.")
}

fn cap7() {
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

fn cap6() {
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixel = 500;
    let pixel_size = wall_size / (canvas_pixel as f64);
    let half = wall_size / 2.;

    let canvas_mutex = Mutex::new(Canvas::new(canvas_pixel, canvas_pixel));

    let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
    sphere.material.color = Color::new(1., 0.2, 1.);
    let ray_origin = Tuple::point(0., 0., -5.);

    let light = Light::point_light(
        Tuple::point(-10., 10., -10.),
        Color::white()
    );

    (0..canvas_pixel)
    .cartesian_product(0..canvas_pixel)
    .par_bridge()
    .for_each(|(x, y)| {
        let world_x = -half + pixel_size * (x as f64);
        let world_y = half - pixel_size * (y as f64);

        let position = Tuple::point(world_x, world_y, wall_z);

        let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

        let xs = sphere.intersect(ray);

        if xs.hit() != None {
            let hit = xs.hit().unwrap();

            let point = ray.position(hit.t);
            let normal = hit.object.normal_at(point);
            let eye = -ray.direction;

            let color = hit.object.material().lighting(hit.object, light, point, eye, normal, true);

            let mut canvas = canvas_mutex.lock().unwrap();
            canvas.set_pixel_color(x, y, color);
        }
    });

    let canvas = canvas_mutex.lock().unwrap();
    let ppm = canvas.to_ppm();
    write("./cap6.ppm", ppm).expect("Error.")
}

fn cap5() {
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixel = 500;
    let pixel_size = wall_size / (canvas_pixel as f64);
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixel, canvas_pixel);
    let red = Color::new(1., 0., 0.);
    let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
    let ray_origin = Tuple::point(0., 0., -5.);

    for y in 0..canvas_pixel {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixel {
            let world_x = -half + pixel_size * (x as f64);

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            sphere.set_transform(Matrix::rotation_z(PI / 4.) * Matrix::scaling(Tuple::vector(0.5, 1., 1.)));

            let xs = sphere.intersect(ray);

            if xs.hit() != None {
                canvas.set_pixel_color(x, y, red);
            }
        }
    }

    let ppm = canvas.to_ppm();
    write("./cap5.ppm", ppm).expect("Error.")
}

fn cap4() {
    let mut canvas = Canvas::new(1000, 1000);
    let color_white = Color::white();

    let origin = Tuple::point(1000./2., 1000./2., 0.);
    let transf_origin = Matrix::translation(origin);

    for h in 0..12 {
        let r = 300.;
        let transf_rotate_z = Matrix::rotation_z(TAU / 12. * (h as f64));
        let point = Tuple::point(0., r, 0.);

        let point_rotate = transf_origin * transf_rotate_z * point;

        let x = point_rotate.x.round() as usize;
        let y = canvas.height - point_rotate.y.round() as usize;
        canvas.set_pixel_color(x, y, color_white);
    }

    let ppm = canvas.to_ppm();
    write("./cap4.ppm", ppm).expect("Error.")
}

fn cap3() {
    let mat4_identity: Matrix<4> = Matrix::identity();

    println!("Mat4 identity: {:?}", mat4_identity);
    println!("Mat4 identity inverse: {:?}", mat4_identity.inverse());

    let mat4: Matrix<4> = Matrix::from([
        [-6., 1., 1., 6.],
        [-8., 5., 8., 6.],
        [-1., 0., 8., 2.],
        [-7., 1., -1., 1.]
    ]);

    println!("Mat4 original: {:?}", mat4);
    println!("Mat4 mult by inverse: {:?}", mat4 * mat4.inverse());
}

fn cap1_cap2() {
    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1., 1., 0.);

    let enviroment = Worldd::world(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0)
    );

    let projectile = ObjectWorld::object(
        Tuple::vector(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25
    );

    let mut current_projectile = projectile;
    while current_projectile.position.y > 0.0 {
        let x = current_projectile.position.x.round() as usize;
        let y = canvas.height - current_projectile.position.y.round() as usize;
        canvas.set_pixel_color(x, y, color);
        current_projectile = tick(&enviroment, &current_projectile);
    }

    let ppm = canvas.to_ppm();
    write("./cap2.ppm", ppm).expect("Error.")
}

pub struct Worldd {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
pub struct ObjectWorld {
    position: Tuple,
    velocity: Tuple,
}

impl ObjectWorld {
    pub fn object(position: Tuple, velocity: Tuple) -> Self{
        ObjectWorld{ position, velocity }
    }
}

impl Worldd {
    pub fn world(gravity: Tuple, wind: Tuple) -> Self{
        Worldd{ gravity, wind }
    }
}

pub fn tick (world: &Worldd, object: &ObjectWorld) -> ObjectWorld{
    ObjectWorld::object(
        object.position + object.velocity,
        object.velocity + world.gravity + world.wind
    )
}