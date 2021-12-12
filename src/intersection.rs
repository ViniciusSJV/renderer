use crate::object::Object;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Object
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
        Intersection { t, object }
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections {
    pub data: Vec<Intersection>
}

impl Intersections {
    pub fn new(mut intersections: Vec<Intersection>) -> Self {
        intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections { data: intersections }
    }

    pub fn hit(&self) -> Option<Intersection> {
        for intersection in self.data.iter() {
            if intersection.t > 0.0 {
                return Some(*intersection);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests_intersection {
    use crate::intersection::{Intersection, Intersections, Object};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::Tuple;

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

    #[test]
    fn the_hit_when_all_intersection_have_positive_t() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let intersect1 = Intersection::new(1.,  Object::from(sphere));
        let intersect2 = Intersection::new(2.,  Object::from(sphere));

        let intersections = Intersections::new(vec![intersect2, intersect1]);

        assert_eq!(intersections.hit(), Some(intersect1));
    }

    #[test]
    fn the_hit_when_some_intersection_have_negative_t() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let intersect1 = Intersection::new(-1.,  Object::from(sphere));
        let intersect2 = Intersection::new(1.,  Object::from(sphere));

        let intersections = Intersections::new(vec![intersect2, intersect1]);

        assert_eq!(intersections.hit(), Some(intersect2));
    }

    #[test]
    fn the_hit_when_all_intersection_have_negative_t() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let intersect1 = Intersection::new(-2.,  Object::from(sphere));
        let intersect2 = Intersection::new(-1.,  Object::from(sphere));

        let intersections = Intersections::new(vec![intersect2, intersect1]);

        assert_eq!(intersections.hit(), None);
    }

    #[test]
    fn the_hit_is_always_the_have_lowest_nonnegative_intersection() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let intersect1 = Intersection::new(5.,  Object::from(sphere));
        let intersect2 = Intersection::new(7.,  Object::from(sphere));
        let intersect3 = Intersection::new(-3.,  Object::from(sphere));
        let intersect4 = Intersection::new(2.,  Object::from(sphere));

        let intersections = Intersections::new(vec![intersect1, intersect2, intersect3, intersect4]);

        assert_eq!(intersections.hit(), Some(intersect4));
    }
}