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

#[cfg(test)]
mod tests_object {
    use crate::intersection::Intersection;
    use crate::object::Object;
    use crate::sphere::Sphere;
    use crate::Tuple;

    #[test]
    pub fn an_intersection_encapsulate_t_and_object() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let object = Object::from(sphere);
        let intersect = Intersection::new(3.5, object);

        assert_eq!(intersect.t, 3.5);
        assert_eq!(intersect.object, Object::from(sphere));
    }
}