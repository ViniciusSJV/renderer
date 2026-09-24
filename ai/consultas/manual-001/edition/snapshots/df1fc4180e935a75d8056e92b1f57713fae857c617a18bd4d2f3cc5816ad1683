use crate::EPSILON;
use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Plane { pub origin: Tuple, pub material: Material, pub transform: Matrix<4>}

impl Plane {
    pub fn new(origin: Tuple) -> Self {
        let transform = Matrix::identity();
        let material = Material::phong();
        Plane { origin, material, transform }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane::new(Tuple::point(0., 0., 0.))
    }
}

impl Intersectable for Plane {
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        if local_ray.direction.y.abs() < EPSILON {
            Intersections::new(vec![])
        } else {
            let t = -local_ray.origin.y / local_ray.direction.y;
            Intersections::new(vec![
                Intersection::new(t,  Object::from(*self))
            ])
        }
    }

    fn local_normal_at(&self, _world_point: Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
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
mod tests_plane {
    use crate::object::{Intersectable, Object};
    use crate::plane::Plane;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let plane = Plane::default();
        let n1 = plane.local_normal_at(Tuple::point(0., 0., 0.));
        let n2 = plane.local_normal_at(Tuple::point(10., 0., -10.));
        let n3 = plane.local_normal_at(Tuple::point(-5., 0., 150.));

        assert_eq!(n1, Tuple::vector(0., 1., 0.));
        assert_eq!(n2, Tuple::vector(0., 1., 0.));
        assert_eq!(n3, Tuple::vector(0., 1., 0.));

    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let plane = Plane::default();
        let ray = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));

        let xs = plane.intersect(ray);
        assert!(xs.data.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let plane = Plane::default();
        let ray = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let xs = plane.intersect(ray);
        assert!(xs.data.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let plane = Plane::default();
        let ray = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));

        let xs = plane.intersect(ray);
        assert_eq!(xs.data.len(), 1);
        assert_eq!(xs.data[0].t, 1.);
        assert_eq!(xs.data[0].object , Object::from(plane));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let plane = Plane::default();
        let ray = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));

        let xs = plane.intersect(ray);
        assert_eq!(xs.data.len(), 1);
        assert_eq!(xs.data[0].t, 1.);
        assert_eq!(xs.data[0].object , Object::from(plane));
    }
}