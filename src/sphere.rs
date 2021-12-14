use crate::ray::Ray;
use crate::{Matrix, Tuple};
use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::object::Object;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere { pub origin: Tuple, pub material: Material, pub transform: Matrix<4>}

impl Sphere {
    pub fn new(origin: Tuple) -> Self {
        let transform = Matrix::identity();
        let material = Material::phong();
        Sphere { origin, material, transform }
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        if !point.is_point() {
            panic!("Normal is only to Tuple::point")
        }
        let object_point = self.transform.inverse() * point;
        let object_normal = object_point - self.origin;
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray_2 = ray.set_transform(self.transform.inverse());
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
                Intersection::new(t1, Object::from(*self), ray),
                Intersection::new(t2, Object::from(*self), ray)
            ])
        }
    }
}

#[cfg(test)]
mod tests_sphere {
    use std::f64::consts::PI;
    use crate::assert_equivalent;
    use crate::equivalent::Equivalence;
    use crate::materials::Material;
    use super::*;

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
        sphere.set_transform(translation);

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

        sphere.set_transform(scaling);
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

        sphere.set_transform(translation);
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let normal = sphere.normal_at(Tuple::point(1., 0., 0.));

        assert_equivalent!(normal, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let normal = sphere.normal_at(Tuple::point(0., 1., 0.));

        assert_equivalent!(normal, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let normal = sphere.normal_at(Tuple::point(0., 0., 1.));

        assert_equivalent!(normal, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let normal = sphere.normal_at(Tuple::point((3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3.));

        assert_equivalent!(normal, Tuple::vector((3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3.));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let normal = sphere.normal_at(Tuple::point((3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3.));

        assert_equivalent!(normal, normal.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
        sphere.set_transform(Matrix::translation(Tuple::vector(0., 1., 0.)));

        let normal = sphere.normal_at(Tuple::point(0., 1.70711, -0.70711));

        assert_equivalent!(normal, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
        sphere.set_transform(
            Matrix::scaling(Tuple::vector(1., 0.5, 1.)) *
            Matrix::rotation_z(PI/5.)
        );

        let normal = sphere.normal_at(Tuple::point(0., (2. as f64).sqrt() / 2., -(2. as f64).sqrt() / 2.));

        assert_equivalent!(normal, Tuple::vector(0., 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_default_material() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.));

        assert_eq!(sphere.material, Material::phong());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.));
        let mut material = Material::phong();
        material.ambient = 1.;

        sphere.material = material;

        assert_eq!(sphere.material, material);
    }
}