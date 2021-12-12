use crate::intersection::Intersections;
use crate::ray::Ray;
use crate::sphere::Sphere;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Sphere(Sphere)
}

impl From<Sphere> for Object {
    fn from(sphere: Sphere) -> Self {
        Object::Sphere(sphere)
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Intersections;
}

impl Intersectable for Object {
    fn intersect(&self, ray: Ray) -> Intersections {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray)
        }
    }
}