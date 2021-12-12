use crate::ray::Ray;
use crate::{Matrix, Tuple};
use crate::intersection::{Intersection, Intersections};
use crate::object::Object;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere { pub origin: Tuple, pub transform: Matrix<4>}

impl Sphere {
    pub fn new(origin: Tuple) -> Self {
        let transform = Matrix::identity();
        Sphere { origin, transform }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = (b * b) - 4. * a * c;

        if discriminant < 0. {
            Intersections::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            Intersections::new(vec![
                Intersection::new(t1, Object::from(*self)),
                Intersection::new(t2, Object::from(*self))
            ])
        }
    }
}

#[cfg(test)]
mod tests_sphere {
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::Tuple;

    #[test]
    fn a_ray_intersection_sphere_at_two_points() {
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );

        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs.data[0].t, 4.);
        assert_eq!(xs.data[1].t, 6.);
    }

    #[test]
    fn a_ray_intersection_sphere_at_two_tangent() {
        let ray = Ray::new(
            Tuple::point(0., 1., -5.),
            Tuple::vector(0., 0., 1.)
        );

        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs.data[0].t, 5.);
        assert_eq!(xs.data[1].t, 5.);
    }

    #[test]
    fn a_ray_missing_sphere() {
        let ray = Ray::new(
            Tuple::point(0., 2., -5.),
            Tuple::vector(0., 0., 1.)
        );

        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 0., 1.)
        );

        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs.data[0].t, -1.);
        assert_eq!(xs.data[1].t, 1.);
    }

    #[test]
    fn a_sphere_is_behing_a_ray() {
        let ray = Ray::new(
            Tuple::point(0., 0., 5.),
            Tuple::vector(0., 0., 1.)
        );

        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs.data[0].t, -6.);
        assert_eq!(xs.data[1].t, -4.);
    }
}