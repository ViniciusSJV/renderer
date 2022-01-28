use crate::EPSILON;
use crate::object::{Intersectable, Object};
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
    pub reflect_v: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub inside: bool
}

impl Computations {
    fn from(intersection: Intersection, ray: Ray) -> Self {
        let t = intersection.t;
        let object = intersection.object;
        let point = ray.position(intersection.t);
        let eye_v = -ray.direction;
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
            reflect_v: ray.direction.reflect(normal_v),
            under_point: Tuple::vector(0., 0., 0.),
            n1: 0.,
            n2: 0.,
            inside
        }
    }
}


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Object
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
        Intersection { t, object }
    }

    pub fn prepare_computations(self, ray: Ray, mut xs: Intersections) -> Computations {
        let mut comps = Computations::from(self, ray);
        for intersect in xs.data.iter() {
            if xs.containers.is_empty() {
                comps.n1 = 1.0;
            } else {
                comps.n1 = xs.containers.last().unwrap().material().reflactive_index;
            }

            if xs.containers.contains(&intersect.object) {
                let index = xs.containers.iter().position(|x| *x == intersect.object).unwrap();
                xs.containers.remove(index);
            } else {
                xs.containers.push(intersect.object);
            }

            if intersect == &self {
                if xs.containers.is_empty() {
                    comps.n2 = 1.0;
                } else {
                    comps.n2 = xs.containers.last().unwrap().material().reflactive_index;
                }
            }
        }
        comps
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections {
    pub data: Vec<Intersection>,
    pub containers: Vec<Object>
}

impl Intersections {
    pub fn new(mut intersections: Vec<Intersection>) -> Self {
        intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections { data: intersections, containers: vec![] }
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
    use crate::object::Intersectable;
    use crate::plane::Plane;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

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

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let s = Object::from(Sphere::default());
        let intersect = Intersection::new(4., s);
        let xs = Intersections::new(vec![intersect]);
        let comps = intersect.prepare_computations(ray, xs);

        assert_eq!(comps.t, intersect.t);
        assert_eq!(comps.point, Tuple::point(0., 0., -1.));
        assert_eq!(comps.eye_v, Tuple::vector(0., 0., -1.));
        assert_eq!(comps.normal_v, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let s = Object::from(Sphere::default());
        let intersect = Intersection::new(4., s);
        let xs = Intersections::new(vec![intersect]);
        let comps = intersect.prepare_computations(ray, xs);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let mut shape = Sphere::default();
        shape.transform = Matrix::translation(Tuple::vector(0., 0., 1.));

        let intersection = Intersection::new(5., Object::from(shape));
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, xs);

        assert!(comps.over_point.z < -EPSILON/2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::default();
        let ray = Ray::new(Tuple::point(0., 1., -1.), Tuple::vector(0., -f64::from(2.).sqrt() / 2., f64::from(2.).sqrt() / 2.));
        let intersection = Intersection::new(f64::from(2.).sqrt(), Object::from(shape));
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, xs);

        assert_eq!(comps.reflect_v, Tuple::vector(0., f64::from(2.).sqrt() / 2., f64::from(2.).sqrt() / 2.));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = Sphere::grass();
        a.set_transform(Matrix::scaling(Tuple::vector(2., 2., 2.)));
        a.material.reflactive_index = 1.5;

        let mut b = Sphere::grass();
        b.set_transform(Matrix::translation(Tuple::vector(0., 0., -0.25)));
        b.material.reflactive_index = 2.;

        let mut c = Sphere::grass();
        c.set_transform(Matrix::translation(Tuple::vector(0., 0., 0.25)));
        c.material.reflactive_index = 2.5;

        let ray = Ray::new(Tuple::point(0., 0., -4.), Tuple::vector(0., 0., 1.));

        let intersect1 = Intersection::new(2.,  Object::from(a));
        let intersect2 = Intersection::new(2.75,  Object::from(b));
        let intersect3 = Intersection::new(3.25,  Object::from(c));
        let intersect4 = Intersection::new(4.75,  Object::from(b));
        let intersect5 = Intersection::new(5.25,  Object::from(c));
        let intersect6 = Intersection::new(6.,  Object::from(a));

        //TODO fix the bug and make pass
        let xs = Intersections::new(vec![intersect1, intersect2, intersect3, intersect4, intersect5, intersect6]);
        let comp0 = xs.data[0].prepare_computations(ray, xs);
        let comp1 = xs.data[1].prepare_computations(ray, xs);
        let comp2 = xs.data[2].prepare_computations(ray, xs);
        let comp3 = xs.data[3].prepare_computations(ray, xs);
        let comp4 = xs.data[4].prepare_computations(ray, xs);
        let comp5 = xs.data[5].prepare_computations(ray, xs);

        assert_eq!(comp0.n1, 1.0);
        assert_eq!(comp0.n2, 1.5);

        assert_eq!(comp1.n1, 1.5);
        assert_eq!(comp1.n2, 2.0);

        assert_eq!(comp2.n1, 2.0);
        assert_eq!(comp2.n2, 2.5);

        assert_eq!(comp3.n1, 2.5);
        assert_eq!(comp3.n2, 2.5);

        assert_eq!(comp4.n1, 2.5);
        assert_eq!(comp4.n2, 1.5);

        assert_eq!(comp5.n1, 1.5);
        assert_eq!(comp5.n2, 1.0);
    }
}