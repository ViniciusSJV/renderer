use std::mem::swap;
use crate::EPSILON;
use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cube {
    pub origin: Tuple, pub material: Material, pub transform: Matrix<4>
}

impl Cube {
    pub fn new(origin: Tuple) -> Self {
        let transform = Matrix::identity();
        let material = Material::phong();
        Cube { origin, material, transform }
    }

    fn check_axis(self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;

        let mut tmin;
        let mut tmax;

        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            swap(&mut tmin, &mut tmax);
        }

        (tmin, tmax)
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::new(Tuple::point(0., 0., 0.))
    }
}

impl Intersectable for Cube {
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let (xt_min, xt_max) = self.check_axis(local_ray.origin.x, local_ray.direction.x);
        let (yt_min, yt_max) = self.check_axis(local_ray.origin.y, local_ray.direction.y);
        let (zt_min, zt_max) = self.check_axis(local_ray.origin.z, local_ray.direction.z);

        let arr_min = vec![xt_min, yt_min, zt_min];
        let arr_max = vec![xt_max, yt_max, zt_max];
        let tmin = arr_min.iter().fold(0.0/0.0, |m, v| v.max(m));
        let tmax = arr_max.iter().fold(0.0/0.0, |m, v| v.min(m));

        if tmin > tmax {
            return Intersections::new(vec![])
        }

        Intersections::new(vec![
            Intersection::new(tmin, Object::from(*self)),
            Intersection::new(tmax, Object::from(*self))
        ])
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let arr_max = vec![local_point.x.abs(), local_point.y.abs(), local_point.z.abs()];
        let max_c = arr_max.iter().fold(0.0/0.0, |m, v| v.max(m));

        if max_c == local_point.x.abs() {
            return Tuple::vector(local_point.x, 0., 0.);
        } else if max_c == local_point.y.abs() {
            return Tuple::vector(0., local_point.y, 0.);
        }

        return Tuple::vector(0., 0., local_point.z);
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
mod tests_cube {
    use crate::cube::Cube;
    use crate::object::Intersectable;
    use crate::ray::Ray;
    use crate::assert_equivalent;
    use crate::equivalent::Equivalence;
    use crate::tuple::Tuple;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::default();

        let ray1 = Ray::new(Tuple::point(5., 0.5, 0.), Tuple::vector(-1.,0., 0.));
        let ray2 = Ray::new(Tuple::point(-5., 0.5, 0.), Tuple::vector(1.,0., 0.));
        let ray3 = Ray::new(Tuple::point(0.5, 5., 0.), Tuple::vector(0.,-1., 0.));
        let ray4 = Ray::new(Tuple::point(0.5, -5., 0.), Tuple::vector(0.,1., 0.));
        let ray5 = Ray::new(Tuple::point(0.5, 0., 5.), Tuple::vector(0.,0., -1.));
        let ray6 = Ray::new(Tuple::point(0.5, 0., -5.), Tuple::vector(0.,0., 1.));

        let ray_inside = Ray::new(Tuple::point(0., 0.5, 0.), Tuple::vector(0.,0., 1.));
        let xs_inside = c.intersect(ray_inside);

        for ray in [ray1, ray2, ray3, ray4, ray5, ray6].iter() {
            let xs = c.intersect(*ray);

            assert_eq!(xs.data.len(), 2);
            assert_eq!(xs.data[0].t, 4.);
            assert_eq!(xs.data[1].t, 6.);
        }

        assert_eq!(xs_inside.data.len(), 2);
        assert_eq!(xs_inside.data[0].t, -1.);
        assert_eq!(xs_inside.data[1].t, 1.);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = Cube::default();

        let ray1 = Ray::new(Tuple::point(-2., 0., 0.), Tuple::vector(0.2673,0.5345, 0.8018));
        let ray2 = Ray::new(Tuple::point(0., -2., 0.), Tuple::vector(0.8018,0.2673, 0.5345));
        let ray3 = Ray::new(Tuple::point(0., 0., -2.), Tuple::vector(0.5345,0.8018, 0.2673));
        let ray4 = Ray::new(Tuple::point(2., 0., 2.), Tuple::vector(0.,0., -1.));
        let ray5 = Ray::new(Tuple::point(0., 2., 2.), Tuple::vector(0.,-1., 0.));
        let ray6 = Ray::new(Tuple::point(2., 2., 0.), Tuple::vector(-1.,0., 0.));

        for ray in [ray1, ray2, ray3, ray4, ray5, ray6].iter() {
            let xs = c.intersect(*ray);

            assert_eq!(xs.data.len(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::default();

        let pont1 = Tuple::point(1., 0.5, -0.8);
        let pont2 = Tuple::point(-1., -0.2, 0.9);
        let pont3 = Tuple::point(-0.4, 1., -0.1);
        let pont4 = Tuple::point(0.3, -1., -0.7);
        let pont5 = Tuple::point(-0.6, 0.3, 1.);
        let pont6 = Tuple::point(0.4, 0.4, -1.);
        let pont7 = Tuple::point(1., 1., 1.);
        let pont8 = Tuple::point(-1., -1., -1.);

        assert_equivalent!(c.local_normal_at(pont1), Tuple::vector(1., 0., 0.));
        assert_equivalent!(c.local_normal_at(pont2), Tuple::vector(-1., 0., 0.));
        assert_equivalent!(c.local_normal_at(pont3), Tuple::vector(0., 1., 0.));
        assert_equivalent!(c.local_normal_at(pont4), Tuple::vector(0., -1., 0.));
        assert_equivalent!(c.local_normal_at(pont5), Tuple::vector(0., 0., 1.));
        assert_equivalent!(c.local_normal_at(pont6), Tuple::vector(0., 0., -1.));
        assert_equivalent!(c.local_normal_at(pont7), Tuple::vector(1., 0., 0.));
        assert_equivalent!(c.local_normal_at(pont8), Tuple::vector(-1., 0., 0.));
    }
}