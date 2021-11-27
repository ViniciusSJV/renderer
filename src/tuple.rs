use std::ops;
use crate::fuzzy_eq::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn tuple(x: f64, y: f64, z:f64, w: f64) -> Self {
        Self { x, y, z, w}
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn is_vector(&self) -> bool {
        self.w.fuzzy_eq(0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w.fuzzy_eq(1.0)
    }
}

impl FuzzyEq<Tuple> for Tuple {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.x.fuzzy_eq(other.x)
            && self.y.fuzzy_eq(other.y)
            && self.z.fuzzy_eq(other.z)
            && self.w.fuzzy_eq(other.w)
    }
}

impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.is_point() && other.is_point() {
            panic!("Add tow points doesn't make sense");
        }
        Tuple::tuple(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple::tuple(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Tuple::tuple(
            0. - self.x,
            0. - self.y,
            0. - self.z,
            0. - self.w
        )
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Tuple::tuple(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other
        )
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Tuple::tuple(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Self) -> Self {
        if !self.is_vector() || !other.is_vector() {
            panic!("Cross product can only be calculated for two vectors.");
        }

        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}

#[cfg(test)]
mod tests_tuple {
    use crate::assert_fuzzy_eq;
    use super::*;

    #[test]
    fn points_does_fill_properties() {
        let point = Tuple::point(4.3, 4.2, 3.1);

        assert_fuzzy_eq!(point.x, 4.3);
        assert_fuzzy_eq!(point.y, 4.2);
        assert_fuzzy_eq!(point.z, 3.1);
        assert_fuzzy_eq!(point.w, 1.);
    }

    #[test]
    fn vector_does_fill_properties() {
        let vector = Tuple::vector(1.4, 8.9, 5.1);

        assert_fuzzy_eq!(vector.x, 1.4);
        assert_fuzzy_eq!(vector.y, 8.9);
        assert_fuzzy_eq!(vector.z, 5.1);
        assert_fuzzy_eq!(vector.w, 0.);
    }

    #[test]
    fn tuple_is_a_point() {
        let point = Tuple::point(1.4, 8.9, 5.1);
        assert!(point.is_point());
    }

    #[test]
    fn tuple_is_a_vector() {
        let vector = Tuple::vector(1.4, 8.9, 5.1);
        assert!(vector.is_vector());
    }

    #[test]
    fn add_to_tuple() {
        let tuple_1 = Tuple::tuple(3.0,-2.0,5.0,1.0);
        let tuple_2 = Tuple::tuple(-2.0,3.0, 1.0,0.0);

        let tuple_expected = Tuple::tuple(1.0,1.0,6.0,1.0);

        assert_fuzzy_eq!(tuple_1 + tuple_2, tuple_expected);
    }

    #[test]
    fn sub_tow_points_to_make_a_vector() {
        let point_1 = Tuple::point(3., 2., 1.);
        let point_2 = Tuple::point(5., 6., 7.);

        let vector_expected = Tuple::vector(-2.,-4.,-6.);

        assert_fuzzy_eq!(point_1 - point_2, vector_expected);
        assert!((point_1 - point_2).is_vector())
    }

    #[test]
    fn sub_a_vector_from_a_point_to_make_a_point() {
        let point = Tuple::point(3.,2.,1.);
        let vector = Tuple::vector(5.,6.,7.);

        let point_expected = Tuple::point(-2.,-4.,-6.);

        assert_fuzzy_eq!(point - vector, point_expected);
        assert!((point - vector).is_point())
    }

    #[test]
    fn sub_tow_vector() {
        let vector_1 = Tuple::vector(3.,2.,1.);
        let vector_2 = Tuple::vector(5.,6.,7.);

        let expected_vector = Tuple::vector(-2.,-4.,-6.);

        assert_fuzzy_eq!(vector_1 - vector_2, expected_vector);
        assert!((vector_1 - vector_2).is_vector())
    }

    #[test]
    fn sub_a_vector_from_a_zero_vector() {
        let vector_1 = Tuple::vector(0.,0.,0.);
        let vector_2 = Tuple::vector(1.,-2.,3.);

        let expected_vector = Tuple::vector(-1.,2.,-3.);

        assert_fuzzy_eq!(vector_1 - vector_2, expected_vector);
    }

    #[test]
    fn neg_tuple() {
        let tuple = Tuple::tuple(1.,-2.,3.,-4.);
        let neg_tuple = Tuple::tuple(-1.,2.,-3.,4.);

        assert_fuzzy_eq!(-tuple, neg_tuple);
    }

    #[test]
    fn mul_tuple_by_scalar() {
        let tuple = Tuple::tuple(1.,-2.,3.,-4.);
        let scarlar = 3.5;
        let expected_tuple = Tuple::tuple(3.5,-7.,10.5,-14.);

        assert_fuzzy_eq!(tuple * scarlar, expected_tuple);
    }

    #[test]
    fn mul_tuple_by_fraction() {
        let tuple = Tuple::tuple(1.,-2.,3.,-4.);
        let fraction = 0.5;
        let expected_tuple = Tuple::tuple(0.5,-1.,1.5,-2.);

        assert_fuzzy_eq!(tuple * fraction, expected_tuple);
    }

    #[test]
    fn div_tuple_by_scalar() {
        let tuple = Tuple::tuple(1.,-2.,3.,-4.);
        let scarlar = 2.0;
        let expected_tuple = Tuple::tuple(0.5,-1.,1.5,-2.);

        assert_fuzzy_eq!(tuple / scarlar, expected_tuple);
    }

    #[test]
    fn get_magnitude_from_vectors() {
        let vec_1 = Tuple::vector(1.,0.,0.);
        let vec_2 = Tuple::vector(0.,1.,0.);
        let vec_3 = Tuple::vector(0.,0.,1.);
        let vec_4 = Tuple::vector(1.,2.,3.);
        let vec_5 = Tuple::vector(-1.,-2.,-3.);

        assert_fuzzy_eq!(vec_1.magnitude(), 1.);
        assert_fuzzy_eq!(vec_2.magnitude(), 1.);
        assert_fuzzy_eq!(vec_3.magnitude(), 1.);
        assert_fuzzy_eq!(vec_4.magnitude(), (14. as f64).sqrt());
        assert_fuzzy_eq!(vec_5.magnitude(), (14. as f64).sqrt());
    }

    #[test]
    fn normalize_two_vectors_in_one_vector() {
        let vec_1 = Tuple::vector(4.,0.,0.);
        let vec_1_expected = Tuple::vector(1.,0.,0.);

        let vec_2 = Tuple::vector(1.,2.,3.);
        let vec_2_expected = Tuple::vector(0.26726,0.53452,0.80178);

        assert_fuzzy_eq!(vec_1.normalize(), vec_1_expected);
        assert!(vec_1.normalize().is_vector());
        assert_fuzzy_eq!(vec_2.normalize(), vec_2_expected);
        assert!(vec_2.normalize().is_vector());
    }

    #[test]
    fn get_magnitude_from_vector_normilize() {
        let vector = Tuple::vector(1.,2.,3.);

        assert_fuzzy_eq!(vector.normalize().magnitude(), 1.);
    }

    #[test]
    fn get_dot_product_of_two_vectors() {
        let vector_1 = Tuple::vector(1.,2.,3.);
        let vector_2 = Tuple::vector(2.,3.,4.);

        assert_fuzzy_eq!(vector_1.dot(vector_2), 20.);
    }

    #[test]
    fn get_cross_of_two_vectors() {
        let vector_1 = Tuple::vector(1.,2.,3.);
        let vector_2 = Tuple::vector(2.,3.,4.);

        let vector_1_expected = Tuple::vector(-1.,2.,-1.);
        let vector_2_expected = Tuple::vector(1.,-2.,1.);

        assert_fuzzy_eq!(vector_1.cross(vector_2), vector_1_expected);
        assert_fuzzy_eq!(vector_2.cross(vector_1), vector_2_expected);

        assert!(vector_1.is_vector());
        assert!(vector_2.is_vector());
    }
}
