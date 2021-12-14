use crate::intersection::Intersections;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

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
        match *self {
            Object::Sphere(ref sphere) => sphere.normal_at(point),
        }
    }
}

#[cfg(test)]
mod tests_object {
    use crate::intersection::Intersection;
    use crate::object::Object;
    use crate::sphere::Sphere;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

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