use crate::ray::Ray;
use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere { pub origin: Tuple, pub material: Material, pub transform: Matrix<4>, pub radius: f64}

impl Sphere {
    pub fn new(origin: Tuple, radius: f64) -> Self {
        let transform = Matrix::identity();
        let material = Material::phong();
        Sphere { origin, material, transform, radius }
    }

    pub fn grass(radius: f64) -> Self {
        let transform = Matrix::identity();
        let material = Material::glass();
        Sphere { origin: Tuple::point(0., 0., 0.), material, transform, radius }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Tuple::point(0., 0., 0.), 1.)
    }
}

impl Intersectable for Sphere {
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let sphere_to_ray = local_ray.origin - self.origin;
        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2. * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - self.radius;

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

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        (local_point - self.origin).normalize()
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
mod tests_sphere {
    use std::f64::consts::PI;
    use std::f64::consts::FRAC_1_SQRT_2;
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

        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
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

        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
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

        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 0., 1.)
        );

        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
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

        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs.data[0].t, -6.);
        assert_eq!(xs.data[1].t, -4.);
    }

    #[test]
    fn a_sphere_default_transformation() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);

        assert_eq!(sphere.transform, Matrix::identity());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
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
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
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
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let translation = Matrix::translation(Tuple::vector(5., 0., 0.));

        sphere.set_transform(translation);
        let xs = sphere.intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let normal = sphere.normal_at(Tuple::point(1., 0., 0.));

        assert_equivalent!(normal, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let normal = sphere.normal_at(Tuple::point(0., 1., 0.));

        assert_equivalent!(normal, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let normal = sphere.normal_at(Tuple::point(0., 0., 1.));

        assert_equivalent!(normal, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let normal = sphere.normal_at(Tuple::point((3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3.));

        assert_equivalent!(normal, Tuple::vector((3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3.));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let normal = sphere.normal_at(Tuple::point((3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3., (3. as f64).sqrt() / 3.));

        assert_equivalent!(normal, normal.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        sphere.set_transform(Matrix::translation(Tuple::vector(0., 1., 0.)));

        let normal = sphere.normal_at(Tuple::point(0., 1.70711, -FRAC_1_SQRT_2));

        assert_equivalent!(normal, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        sphere.set_transform(
            Matrix::scaling(Tuple::vector(1., 0.5, 1.)) *
            Matrix::rotation_z(PI/5.)
        );

        let normal = sphere.normal_at(Tuple::point(0., (2. as f64).sqrt() / 2., -(2. as f64).sqrt() / 2.));

        assert_equivalent!(normal, Tuple::vector(0., 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_default_material() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);

        assert_eq!(sphere.material, Material::phong());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let mut material = Material::phong();
        material.ambient = 1.;

        sphere.material = material;

        assert_eq!(sphere.material, material);
    }

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let sphere = Sphere::grass(1.);
        assert_eq!(sphere.transform(), Matrix::identity());
        assert_eq!(sphere.material.transparency, 1.0);
        assert_eq!(sphere.material.reflactive_index, 1.5);
    }
}