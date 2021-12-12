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

    pub fn transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray_2 = ray.transform(self.transform.inverse());
        let sphere_to_ray = ray_2.origin - self.origin;
        let a = ray_2.direction.dot(ray_2.direction);
        let b = 2. * ray_2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

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
    use crate::{Matrix, Tuple};

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

    #[test]
    fn a_sphere_default_transformation() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        assert_eq!(sphere.transform, Matrix::identity());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let translation = Matrix::translation(Tuple::vector(2., 3.,4.));
        sphere.transform(translation);

        assert_eq!(sphere.transform, translation);
    }

    #[test]
    fn intersecting_a_scale_sphere_with_a_ray() {
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let scaling = Matrix::scaling(Tuple::vector(2., 2., 2.));

        sphere.transform(scaling);
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs.data[0].t, 3.);
        assert_eq!(xs.data[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let translation = Matrix::translation(Tuple::vector(5., 0., 0.));

        sphere.transform(translation);
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }
}