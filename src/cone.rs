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
pub struct Cone {
    pub origin: Tuple, pub material: Material, pub transform: Matrix<4>, pub minimum: f64, pub maximum: f64, closed: bool
}

impl Cone {
    pub fn new(origin: Tuple) -> Self {
        let transform = Matrix::identity();
        let material = Material::phong();
        let minimum =  -f64::INFINITY;
        let maximum = f64::INFINITY;
        Cone { origin, material, transform, minimum, maximum, closed: false }
    }

    fn check_cap(self, ray: Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        (x.powi(2) + z.powi(2)) <= (ray.origin.y + t * ray.direction.y).abs()
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

impl Default for Cone {
    fn default() -> Self {
        Cone::new(Tuple::point(0., 0., 0.))
    }
}

impl Intersectable for Cone {
    fn local_intersect(&self, local_ray: Ray) -> Intersections {

        let a = local_ray.direction.x * local_ray.direction.x - local_ray.direction.y * local_ray.direction.y +
            local_ray.direction.z * local_ray.direction.z;

        let b = 2.0 * local_ray.origin.x * local_ray.direction.x - 2.0 * local_ray.origin.y * local_ray.direction.y +
            2.0 * local_ray.origin.z * local_ray.direction.z;

        let c = local_ray.origin.x * local_ray.origin.x - local_ray.origin.y * local_ray.origin.y + local_ray.origin.z * local_ray.origin.z;

        let mut xs: Vec<Intersection> = vec![];

        if a.equivalent(0.) {
            if b.equivalent(0.) {
                return Intersections::new(vec![]);
            }
            let t = -c / (2. * b);
            xs.push(Intersection::new(t, Object::from(*self)));
        }

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            return Intersections::new(xs);
        }

        let mut t0 = (-b - disc.sqrt()) / (2. * a);
        let mut t1 = (-b + disc.sqrt()) / (2. * a);

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        let y0 = local_ray.origin.y + t0 * local_ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t0, Object::from(*self)));
        }

        let y1 = local_ray.origin.y + t1 * local_ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t1, Object::from(*self)));
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

        let mut y = (local_point.x.powi(2) + local_point.z.powi(2)).sqrt();
        if local_point.y > 0. {
            y = -y;
        }

        Tuple::vector(local_point.x, y, local_point.z)
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
mod tests_cone {
    use crate::cone::Cone;
    use crate::ray::Ray;
    use crate::tuple::Tuple;
    use crate::assert_equivalent;
    use crate::equivalent::Equivalence;
    use crate::object::Intersectable;

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let cone = Cone::default();

        let direction1 = Tuple::vector(0., 0., 1.).normalize();
        let direction2 = Tuple::vector(1., 1., 1.).normalize();
        let direction3 = Tuple::vector(-0.5, -1., 1.).normalize();

        let ray1 = Ray::new(Tuple::point(0., 0., -5.), direction1);
        let ray2 = Ray::new(Tuple::point(0., 0., -5.), direction2);
        let ray3 = Ray::new(Tuple::point(1., 1., -5.), direction3);

        let xs1 = cone.intersect(ray1);
        let xs2 = cone.intersect(ray2);
        let xs3 = cone.intersect(ray3);

        assert_eq!(xs1.data.len(), 2);
        assert_equivalent!(xs1.data[0].t, 5.);
        assert_equivalent!(xs1.data[1].t, 5.);

        assert_eq!(xs2.data.len(), 2);
        assert_equivalent!(xs2.data[0].t, 8.66025);
        assert_equivalent!(xs2.data[1].t, 8.66025);

        assert_eq!(xs3.data.len(), 2);
        assert_equivalent!(xs3.data[0].t, 4.55006);
        assert_equivalent!(xs3.data[1].t, 49.44994);
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let cone = Cone::default();
        let direction = Tuple::vector(0., 1., 1.).normalize();
        let ray = Ray::new(Tuple::point(0., 0., -1.), direction);
        let xs = cone.intersect(ray);

        assert_eq!(xs.data.len(), 1);
        assert_equivalent!(xs.data[0].t, 0.35355);
    }

    #[test]
    fn intersecting_a_cone_end_caps() {
        let mut cone = Cone::default();
        cone.minimum = -0.5;
        cone.maximum = 0.5;
        cone.closed = true;

        let direction1 = Tuple::vector(0., 1., 0.).normalize();
        let direction2 = Tuple::vector(0., 1., 1.).normalize();
        let direction3 = Tuple::vector(0., 1., 0.).normalize();

        let ray1 = Ray::new(Tuple::point(0., 0., -5.), direction1);
        let ray2 = Ray::new(Tuple::point(0., 0., -0.25), direction2);
        let ray3 = Ray::new(Tuple::point(0., 0., -0.25), direction3);

        let xs1 = cone.intersect(ray1);
        let xs2 = cone.intersect(ray2);
        let xs3 = cone.intersect(ray3);

        assert_eq!(xs1.data.len(), 0);
        assert_eq!(xs2.data.len(), 2);
        assert_eq!(xs3.data.len(), 4);
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let cone = Cone::default();

        assert_equivalent!(cone.local_normal_at(Tuple::point(0., 0., 0.)), Tuple::vector(0., 0., 0.));
        assert_equivalent!(cone.local_normal_at(Tuple::point(1., 1., 1.)), Tuple::vector(1., -f64::from(2.).sqrt(), 1.));
        assert_equivalent!(cone.local_normal_at(Tuple::point(-1., -1., 0.)), Tuple::vector(-1., 1., 0.));
    }
}