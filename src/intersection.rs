use crate::EPSILON;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Computations {
    pub t: f64,
    pub object: Object,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool
}

impl From<Intersection> for Computations {
    fn from(intersection: Intersection) -> Self {
        let t = intersection.t;
        let object = intersection.object;
        let point = intersection.ray.position(intersection.t);
        let eye_v = -intersection.ray.direction;
        let mut normal_v = intersection.object.normal_at(point);
        let mut inside = false;
        if normal_v.dot(eye_v) < 0. {
            inside = true;
            normal_v = -normal_v;
        }
        Computations {
            t,
            object,
            point,
            over_point: point + normal_v * EPSILON,
            eye_v,
            normal_v,
            inside
        }
    }
}


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub ray: Ray,
    pub object: Object
}

impl Intersection {
    pub fn new(t: f64, object: Object, ray: Ray) -> Self {
        Intersection { t, ray, object }
    }

    pub fn prepare_computations(self) -> Computations {
        Computations::from(self)
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

impl IntoIterator for Intersections {
    type Item = Intersection;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests_intersection {
    use crate::EPSILON;
    use crate::intersection::{Intersection, Intersections, Object};
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    pub fn aggregating_intersection() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let ray = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersect1 = Intersection::new(1.,  Object::from(sphere), ray);
        let intersect2 = Intersection::new(2.,  Object::from(sphere), ray);

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

        let ray = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersect1 = Intersection::new(1.,  Object::from(sphere), ray);
        let intersect2 = Intersection::new(2.,  Object::from(sphere), ray);

        let intersections = Intersections::new(vec![intersect2, intersect1]);

        assert_eq!(intersections.hit(), Some(intersect1));
    }

    #[test]
    fn the_hit_when_some_intersection_have_negative_t() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let ray = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersect1 = Intersection::new(-1.,  Object::from(sphere), ray);
        let intersect2 = Intersection::new(1.,  Object::from(sphere), ray);

        let intersections = Intersections::new(vec![intersect2, intersect1]);

        assert_eq!(intersections.hit(), Some(intersect2));
    }

    #[test]
    fn the_hit_when_all_intersection_have_negative_t() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let ray = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersect1 = Intersection::new(-2.,  Object::from(sphere), ray);
        let intersect2 = Intersection::new(-1.,  Object::from(sphere), ray);

        let intersections = Intersections::new(vec![intersect2, intersect1]);

        assert_eq!(intersections.hit(), None);
    }

    #[test]
    fn the_hit_is_always_the_have_lowest_nonnegative_intersection() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        let ray = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersect1 = Intersection::new(5.,  Object::from(sphere), ray);
        let intersect2 = Intersection::new(7.,  Object::from(sphere), ray);
        let intersect3 = Intersection::new(-3.,  Object::from(sphere), ray);
        let intersect4 = Intersection::new(2.,  Object::from(sphere), ray);

        let intersections = Intersections::new(vec![intersect1, intersect2, intersect3, intersect4]);

        assert_eq!(intersections.hit(), Some(intersect4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let s = Object::from(Sphere::default());
        let i = Intersection::new(4., s, ray);

        let compss = i.prepare_computations();

        assert_eq!(compss.t, i.t);
        assert_eq!(compss.point, Tuple::point(0., 0., -1.));
        assert_eq!(compss.eye_v, Tuple::vector(0., 0., -1.));
        assert_eq!(compss.normal_v, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let s = Object::from(Sphere::default());
        let i = Intersection::new(4., s, ray);

        let compss = i.prepare_computations();

        assert!(!compss.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let mut shape = Sphere::default();
        shape.transform = Matrix::translation(Tuple::vector(0., 0., 1.));

        let intersection = Intersection::new(5., Object::from(shape), ray);
        let comps = intersection.prepare_computations();

        assert!(comps.over_point.z < -EPSILON/2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}