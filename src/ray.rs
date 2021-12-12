use crate::Tuple;

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
}

#[cfg(test)]
mod tests_ray {
    use crate::ray::Ray;
    use crate::Tuple;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distence() {
        let origin = Tuple::point(2., 3., 4.);
        let direction = Tuple::vector(1., 0., 0.);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.position(0.), Tuple::point(2., 3., 4.));
        assert_eq!(ray.position(1.), Tuple::point(3., 3., 4.));
        assert_eq!(ray.position(-1.), Tuple::point(1., 3., 4.));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3., 4.));
    }
}