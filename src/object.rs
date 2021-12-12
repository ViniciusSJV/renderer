use crate::intersection::Intersections;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::{Intersection, Matrix, Tuple};
use crate::materials::Material;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Sphere(Sphere)
}

impl From<Sphere> for Object {
    fn from(sphere: Sphere) -> Self {
        Object::Sphere(sphere)
    }
}

impl Object {
    pub fn material(&self) -> Material {
        match *self {
            Object::Sphere(ref sphere) => sphere.material,
        }
    }

    pub fn transform(&self) -> Matrix<4> {
        match *self {
            Object::Sphere(ref sphere) => sphere.transform,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray),
        }
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        if !point.is_point() {
            panic!("Normal is only to Tuple::point")
        }
        let object_point = self.transform().inverse() * point;
        let object_normal = object_point - Tuple::point(0., 0., 0.);
        let mut world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests_object {
    use crate::intersection::Intersection;
    use crate::object::Object;
    use crate::sphere::Sphere;
    use crate::{Ray, Tuple};

    #[test]
    pub fn an_intersection_encapsulate_t_and_object() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let object = Object::from(sphere);

        let ray = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersect = Intersection::new(3.5, object, ray);

        assert_eq!(intersect.t, 3.5);
        assert_eq!(intersect.object, Object::from(sphere));
    }
}