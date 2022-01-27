use std::sync::{Mutex};
use itertools::Itertools;
use rayon::prelude::*;
use crate::canvas::Canvas;
use crate::equivalent::Equivalence;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    pub horizontal_size: usize,
    pub vertical_size: usize,
    pub field_of_view: f64,
    pub transform: Matrix<4>,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    remaining: usize
}

impl Camera {
    pub fn new(horizontal_size: usize, vertical_size: usize, field_of_view: f64) -> Self {
        let half_size = (field_of_view / 2.0).tan();
        let aspect_ratio = horizontal_size as f64 / vertical_size as f64;
        let half_width;
        let half_height;

        if aspect_ratio >= 1.0 {
            half_width = half_size;
            half_height = half_size / aspect_ratio;
        } else {
            half_height = half_size;
            half_width = half_size * aspect_ratio;
        }

        let pixel_size = (half_width * 2.0) / horizontal_size as f64;

        Self {
            horizontal_size,
            vertical_size,
            field_of_view,
            transform: Matrix::identity(),
            half_width,
            half_height,
            pixel_size,
            remaining: 4
        }
    }

    pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self
    }

    pub fn ray_from_pixel(self, x: usize, y: usize) -> Ray {
        let offset_x = (0.5 + x as f64) * self.pixel_size;
        let offset_y = (0.5 + y as f64) * self.pixel_size;

        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        let inverse_camera_transform = self.transform.inverse();

        let pixel = inverse_camera_transform * Tuple::point(world_x, world_y, -1.0);
        let origin = inverse_camera_transform * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(self, world: World) -> Canvas {
        let canvas_mutex = Mutex::new(Canvas::new(self.horizontal_size, self.vertical_size));
        (0..self.horizontal_size)
            .cartesian_product(0..self.vertical_size)
            .par_bridge()
            .for_each(|(x, y)| {
                let ray = self.ray_from_pixel(x, y);
                let color = world.clone().color_at(ray, self.remaining);
                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.set_pixel_color(x, y, color);
            });
        let canvas = canvas_mutex.into_inner().unwrap();
        canvas
    }
}

impl Equivalence<Camera> for Camera {
    fn equivalent(&self, other: Camera) -> bool {
        self.transform.equivalent(other.transform)
            && self.vertical_size == other.vertical_size
            && self.horizontal_size == other.horizontal_size
            && self.field_of_view.equivalent(other.field_of_view)
    }
}

#[cfg(test)]
mod tests_camera {
    use std::f64::consts::PI;
    use crate::assert_equivalent;
    use crate::color::Color;
    use crate::lights::Light;
    use crate::materials::Material;
    use crate::object::{Intersectable, Object};
    use crate::sphere::Sphere;
    use crate::transformations::Transform;
    use crate::tuple::Tuple;
    use crate::world::World;
    use super::*;

    #[test]
    fn constructing_a_camera() {
        let h_size: usize = 160;
        let v_size: usize = 120;
        let field_of_view: f64 = PI/2.;

        let c = Camera::new(h_size, v_size, field_of_view);

        assert_eq!(c.horizontal_size, 160);
        assert_eq!(c.vertical_size, 120);
        assert_eq!(c.field_of_view, PI/2.);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI/2.);
        assert_equivalent!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI/2.);
        assert_equivalent!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI/2.);

        let ray = c.ray_from_pixel(100, 50);

        assert_equivalent!(ray.origin, Tuple::point(0., 0., 0.));
        assert_equivalent!(ray.direction, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI/2.);

        let ray = c.ray_from_pixel(0, 0);

        assert_equivalent!(ray.origin, Tuple::point(0., 0., 0.));
        assert_equivalent!(ray.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(201, 101, PI/2.).with_transform(
            Matrix::rotation_y(PI/4.) * Matrix::translation(Tuple::vector(0., -2., 5.))
        );

        let ray = c.ray_from_pixel(100, 50);

        assert_equivalent!(ray.origin, Tuple::point(0., 2., -5.));
        assert_equivalent!(ray.direction, Tuple::vector((2.0 as f64).sqrt() / 2.0, 0., -(2.0 as f64).sqrt() / 2.0));
    }

    fn create_default_world() -> World {
        let light = Light::point_light(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut material = Material::phong();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;

        let mut sphere = Sphere::default();
        sphere.set_material(material);

        let mut sphere_2 = Sphere::default();
        sphere_2.set_transform(Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5)));

        let s1 = Object::from(sphere);
        let s2 = Object::from(sphere_2);

        World::new(vec![s1, s2], vec![light])
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let world = create_default_world();
        let from = Tuple::point(0., 0., -5.);
        let to  = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);

        let camera = Camera::new(11, 11, PI/2.).with_transform(
            from.view_transform(to, up)
        );

        let canvas = camera.render(world);

        assert_equivalent!(canvas.get_pixel_color(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}