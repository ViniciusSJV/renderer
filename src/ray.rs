use crate::{Matrix, Tuple};

#[derive(PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self{
        if !origin.is_point() || !direction.is_vector() {
            panic!("Invalid args. to Ray. origin = Tuple::point | direction = Tuple::vector")
        }
        Ray {origin, direction}
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn set_transform(&self, m: Matrix<4>) -> Self {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests_ray {
    use crate::ray::Ray;
    use crate::{assert_equivalent, Matrix, Tuple};
    use crate::equivalent::Equivalence;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);

        let ray = Ray::new(origin, direction);

        assert_equivalent!(ray.origin, origin);
        assert_equivalent!(ray.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distence() {
        let origin = Tuple::point(2., 3., 4.);
        let direction = Tuple::vector(1., 0., 0.);

        let ray = Ray::new(origin, direction);

        assert_equivalent!(ray.position(0.), Tuple::point(2., 3., 4.));
        assert_equivalent!(ray.position(1.), Tuple::point(3., 3., 4.));
        assert_equivalent!(ray.position(-1.), Tuple::point(1., 3., 4.));
        assert_equivalent!(ray.position(2.5), Tuple::point(4.5, 3., 4.));
    }

    #[test]
    fn translating_a_ray() {
        let ray = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let translation = Matrix::translation(Tuple::vector(3., 4., 5.));

        let ray_transform = ray.set_transform(translation);

        assert_equivalent!(ray_transform.origin, Tuple::point(4., 6., 8.), );
        assert_equivalent!(ray_transform.direction, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn scaling_a_ray() {
        let ray = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let scaling = Matrix::scaling(Tuple::vector(2., 3., 4.));
        let r2_scaling = ray.set_transform(scaling);

        assert_equivalent!(r2_scaling.origin, Tuple::point(2., 6., 12.), );
        assert_equivalent!(r2_scaling.direction, Tuple::vector(0., 3., 0.));
    }
}