use std::mem::swap;
use crate::EPSILON;
use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::equivalent::Equivalence;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cylinder {
    pub origin: Tuple, pub material: Material, pub transform: Matrix<4>, pub minimum: f64, pub maximum: f64, pub closed: bool
}

impl Cylinder {
    pub fn new(origin: Tuple) -> Self {
        let transform = Matrix::identity();
        let material = Material::phong();
        let minimum =  -f64::INFINITY;
        let maximum = f64::INFINITY;
        Cylinder { origin, material, transform, minimum, maximum, closed: false }
    }

    fn check_cap(self, ray: Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        (x.powi(2) + z.powi(2)) <= 1.
    }

    fn intersect_caps(&self, ray: Ray, xs: &mut Intersections) {
        if !self.closed || ray.direction.y.equivalent(0.) {
            return;
        }

        let mut t = (self.minimum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t) {
            xs.data.push(Intersection::new(t, Object::from(*self)));
        }

        t = (self.maximum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t) {
            xs.data.push(Intersection::new(t, Object::from(*self)));
        }
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder::new(Tuple::point(0., 0., 0.))
    }
}

impl Intersectable for Cylinder {
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let a = local_ray.direction.x.powi(2) + local_ray.direction.z.powi(2);

        if a.equivalent(0.) {
            let mut intersections = Intersections::new(vec![]);
            self.intersect_caps(local_ray, &mut intersections);
            return intersections;
        }

        let b = 2. * local_ray.origin.x * local_ray.direction.x +
                2. * local_ray.origin.z * local_ray.direction.z;

        let c = local_ray.origin.x.powi(2) + local_ray.origin.z.powi(2) - 1.;

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            return Intersections::new(vec![]);
        }

        let mut t0 = (-b - disc.sqrt()) / (2. * a);
        let mut t1 = (-b + disc.sqrt()) / (2. * a);

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        let mut xs: Vec<Intersection> = vec![];

        let y0 = local_ray.origin.y + t0 * local_ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t0, Object::from(*self)))
        }

        let y1 = local_ray.origin.y + t1 * local_ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t1, Object::from(*self)))
        }

        let mut intersections = Intersections::new(xs);

        self.intersect_caps(local_ray, &mut intersections);
        intersections
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);

        if dist < 1. && local_point.y >= self.maximum - EPSILON {
            return Tuple::vector(0., 1., 0.);
        } else if dist < 1. && local_point.y <= self.minimum + EPSILON {
            return Tuple::vector(0., -1., 0.);
        }

        Tuple::vector(local_point.x, 0., local_point.z)
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_material(&mut self, material: Material) {
        self.material = material
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }
}

#[cfg(test)]
mod tests_cylinder {
    use crate::cylinder::Cylinder;
    use crate::object::Intersectable;
    use crate::ray::Ray;
    use crate::tuple::Tuple;
    use crate::assert_equivalent;
    use crate::equivalent::Equivalence;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = Cylinder::default();

        let direction1 = Tuple::vector(0., 1., 0.).normalize();
        let direction2 = Tuple::vector(0., 1., 0.).normalize();
        let direction3 = Tuple::vector(1., 1., 1.).normalize();

        let ray1 = Ray::new(Tuple::point(1., 0., 0.), direction1);
        let ray2 = Ray::new(Tuple::point(0., 0., 0.), direction2);
        let ray3 = Ray::new(Tuple::point(0., 0., -5.), direction3);

        for ray in [ray1, ray2, ray3].iter() {
            let xs = cyl.intersect(*ray);

            assert_eq!(xs.data.len(), 0);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = Cylinder::default();

        let direction1 = Tuple::vector(0., 0., 1.).normalize();
        let direction2 = Tuple::vector(0., 0., 1.).normalize();
        let direction3 = Tuple::vector(0.1, 1., 1.).normalize();

        let ray1 = Ray::new(Tuple::point(1., 0., -5.), direction1);
        let ray2 = Ray::new(Tuple::point(0., 0., -5.), direction2);
        let ray3 = Ray::new(Tuple::point(0.5, 0., -5.), direction3);

        let xs1 = cyl.intersect(ray1);
        let xs2 = cyl.intersect(ray2);
        let xs3 = cyl.intersect(ray3);

        assert_eq!(xs1.data.len(), 2);
        assert_eq!(xs2.data.len(), 2);
        assert_eq!(xs3.data.len(), 2);

        assert_eq!(xs1.data[0].t, 5.);
        assert_eq!(xs1.data[1].t, 5.);

        assert_eq!(xs2.data[0].t, 4.);
        assert_eq!(xs2.data[1].t, 6.);

        assert_equivalent!(xs3.data[0].t, 6.80798);
        assert_equivalent!(xs3.data[1].t, 7.08872);
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Cylinder::default();

        assert_equivalent!(cyl.local_normal_at(Tuple::point(1., 0., 0.)), Tuple::vector(1., 0., 0.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(0., 5., -1.)), Tuple::vector(0., 0., -1.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(0., -2., 1.)), Tuple::vector(0., 0., 1.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(-1., 1., 0.)), Tuple::vector(-1., 0., 0.));
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::default();

        assert_eq!(cyl.minimum, -f64::INFINITY);
        assert_eq!(cyl.maximum, f64::INFINITY);
    }

    #[test]
    fn intersect_a_constrained_cylinder() {
        let mut cyl = Cylinder::default();
        cyl.minimum = 1.;
        cyl.maximum = 2.;

        let direction1 = Tuple::vector(0.1, 1., 0.).normalize();
        let direction2 = Tuple::vector(0., 0., 1.).normalize();
        let direction3 = Tuple::vector(0., 0., 1.).normalize();
        let direction4 = Tuple::vector(0., 0., 1.).normalize();
        let direction5 = Tuple::vector(0., 0., 1.).normalize();
        let direction6 = Tuple::vector(0., 0., 1.).normalize();

        let ray1 = Ray::new(Tuple::point(0., 1.5, 0.), direction1);
        let ray2 = Ray::new(Tuple::point(0., 3., -5.), direction2);
        let ray3 = Ray::new(Tuple::point(0., 0., -5.), direction3);
        let ray4 = Ray::new(Tuple::point(0., 2., -5.), direction4);
        let ray5 = Ray::new(Tuple::point(0., 1., -5.), direction5);
        let ray6 = Ray::new(Tuple::point(0., 1.5, -2.), direction6);

        let xs1 = cyl.intersect(ray1);
        let xs2 = cyl.intersect(ray2);
        let xs3 = cyl.intersect(ray3);
        let xs4 = cyl.intersect(ray4);
        let xs5 = cyl.intersect(ray5);
        let xs6 = cyl.intersect(ray6);

        assert_eq!(xs1.data.len(), 0);
        assert_eq!(xs2.data.len(), 0);
        assert_eq!(xs3.data.len(), 0);
        assert_eq!(xs4.data.len(), 0);
        assert_eq!(xs5.data.len(), 0);
        assert_eq!(xs6.data.len(), 2);
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::default();
        assert!(!cyl.closed);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let mut cyl = Cylinder::default();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        cyl.closed = true;

        let direction1 = Tuple::vector(0., -1., 0.).normalize();
        let direction2 = Tuple::vector(0., -1., 2.).normalize();
        let direction3 = Tuple::vector(0., -1., 1.).normalize();
        let direction4 = Tuple::vector(0., 1., 2.).normalize();
        let direction5 = Tuple::vector(0., 1., 1.).normalize();

        let ray1 = Ray::new(Tuple::point(0., 3., 0.), direction1);
        let ray2 = Ray::new(Tuple::point(0., 3., -2.), direction2);
        let ray3 = Ray::new(Tuple::point(0., 4., -2.), direction3);
        let ray4 = Ray::new(Tuple::point(0., 0., -2.), direction4);
        let ray5 = Ray::new(Tuple::point(0., -1., -2.), direction5);

        for ray in [ray1, ray2, ray3, ray4, ray5].iter() {
            let xs = cyl.intersect(*ray);

            assert_eq!(xs.data.len(), 2);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinder_end_caps() {
        let mut cyl = Cylinder::default();
        cyl.minimum = 1.;
        cyl.maximum = 2.;
        cyl.closed = true;

        assert_equivalent!(cyl.local_normal_at(Tuple::point(0., 1., 0.)), Tuple::vector(0., -1., 0.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(0.5, 1., 0.)), Tuple::vector(0., -1., 0.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(0., 1., 0.5)), Tuple::vector(0., -1., 0.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(0., 2., 0.)), Tuple::vector(0., 1., 0.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(0.5, 2., 0.)), Tuple::vector(0., 1., 0.));
        assert_equivalent!(cyl.local_normal_at(Tuple::point(0., 2., 0.5)), Tuple::vector(0., 1., 0.));
    }
}