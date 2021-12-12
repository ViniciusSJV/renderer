use crate::object::Object;

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Object
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self{
        Intersection {t, object}
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections {
    pub data: Vec<Intersection>
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Intersections { data: intersections}
    }
}

#[cfg(test)]
mod tests_intersection {
    use crate::intersection::{Intersection, Intersections, Object};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::Tuple;

    #[test]
    pub fn an_intersection_encapsulate_t_and_object() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let object = Object::from(sphere);
        let intersect = Intersection::new(3.5,  object);

        assert_eq!(intersect.t, 3.5);
        assert_eq!(intersect.object, Object::from(sphere));
    }

    #[test]
    pub fn aggregating_intersection() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let intersect1 = Intersection::new(1.,  Object::from(sphere));
        let intersect2 = Intersection::new(2.,  Object::from(sphere));

        let intersections = Intersections::new(vec![intersect1, intersect2]);

        assert_eq!(intersections.data.len(), 2);
        assert_eq!(intersections.data[0].object, Object::from(sphere));
        assert_eq!(intersections.data[1].object, Object::from(sphere));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let xs = sphere.intersect(ray);
        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs.data[0].object, Object::from(sphere));
        assert_eq!(xs.data[1].object, Object::from(sphere));
    }
}