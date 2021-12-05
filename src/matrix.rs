//https://docs.rs/cgmath/0.12.0/cgmath/struct.Matrix4.html

use std::ops;
use crate::equivalent::*;

type Mat2 = [[f64; 2]; 2];
type Mat3 = [[f64; 3]; 3];
type Mat4 = [[f64; 4]; 4];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2 {
    pub data: Mat2
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix3 {
    pub data: Mat3
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4 {
    pub data: Mat4
}

impl Matrix2 {
    pub fn new(data: Mat2) -> Matrix2 {
        Matrix2{data}
    }
}

impl Matrix3 {
    pub fn new(data: Mat3) -> Matrix3 {
        Matrix3{data}
    }
}

impl Matrix4 {
    pub fn new(data: Mat4) -> Matrix4 {
        Matrix4{data}
    }
}

impl Equivalence<Matrix2> for Matrix2 {
    fn equivalent(&self, other: Self) -> bool {
        self.data[0][0].equivalent(other.data[0][0])
            && self.data[0][1].equivalent(other.data[0][1])

            && self.data[1][0].equivalent(other.data[1][0])
            && self.data[1][1].equivalent(other.data[1][1])
    }
}

impl Equivalence<Matrix3> for Matrix3 {
    fn equivalent(&self, other: Self) -> bool {
        self.data[0][0].equivalent(other.data[0][0])
            && self.data[0][1].equivalent(other.data[0][1])
            && self.data[0][2].equivalent(other.data[0][2])

            && self.data[1][0].equivalent(other.data[1][0])
            && self.data[1][1].equivalent(other.data[1][1])
            && self.data[1][2].equivalent(other.data[1][2])

            && self.data[2][0].equivalent(other.data[2][0])
            && self.data[2][1].equivalent(other.data[2][1])
            && self.data[2][2].equivalent(other.data[2][2])
    }
}

impl Equivalence<Matrix4> for Matrix4 {
    fn equivalent(&self, other: Self) -> bool {
        self.data[0][0].equivalent(other.data[0][0])
            && self.data[0][1].equivalent(other.data[0][1])
            && self.data[0][2].equivalent(other.data[0][2])
            && self.data[0][3].equivalent(other.data[0][3])

            && self.data[1][0].equivalent(other.data[1][0])
            && self.data[1][1].equivalent(other.data[1][1])
            && self.data[1][2].equivalent(other.data[1][2])
            && self.data[1][3].equivalent(other.data[1][3])

            && self.data[2][0].equivalent(other.data[2][0])
            && self.data[2][1].equivalent(other.data[2][1])
            && self.data[2][2].equivalent(other.data[2][2])
            && self.data[2][3].equivalent(other.data[2][3])

            && self.data[3][0].equivalent(other.data[3][0])
            && self.data[3][1].equivalent(other.data[3][1])
            && self.data[3][2].equivalent(other.data[3][2])
            && self.data[3][3].equivalent(other.data[3][3])
    }
}

//https://www.todamateria.com.br/multiplicacao-de-matrizes/
impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, other: Matrix4) -> Self {
        Matrix4::new(other.data);
    }
}

#[cfg(test)]
mod tests_tuple {
    use super::*;

    #[test]
    fn constructing_matrix_2_x_2 () {
        let data: Mat2 = [
            [-3.0, 5.0],
            [1.0, -2.0]
        ];

        let mat2: Matrix2 = Matrix2::new(data);

        assert_eq!(mat2.data[0][0], -3.0);
        assert_eq!(mat2.data[0][1], 5.0);
        assert_eq!(mat2.data[1][0], 1.0);
        assert_eq!(mat2.data[1][1], -2.0);
    }

    #[test]
    fn constructing_matrix_3_x_3 () {
        let data: Mat3 = [
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]
        ];

        let mat3: Matrix3 = Matrix3::new(data);

        assert_eq!(mat3.data[0][0], -3.0);
        assert_eq!(mat3.data[1][1], -2.0);
        assert_eq!(mat3.data[2][2], 1.0);
    }

    #[test]
    fn constructing_matrix_4_x_4 () {
        let data: Mat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]
        ];

        let mat4: Matrix4 = Matrix4::new(data);

        assert_eq!(mat4.data[0][0], 1.0);
        assert_eq!(mat4.data[0][3], 4.0);
        assert_eq!(mat4.data[1][0], 5.5);
        assert_eq!(mat4.data[1][2], 7.5);
        assert_eq!(mat4.data[2][2], 11.0);
        assert_eq!(mat4.data[3][0], 13.5);
        assert_eq!(mat4.data[3][2], 15.5);
    }

    #[test]
    fn matrix_equality_identical_matrices () {
        let data_1: Mat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ];

        let data_2: Mat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ];

        let mat4_1: Matrix4 = Matrix4::new(data_1);
        let mat4_2: Matrix4 = Matrix4::new(data_2);

        assert_eq!(mat4_1.equivalent(mat4_2), true);
        assert!(mat4_1.equivalent(mat4_2));
    }

    #[test]
    fn matrix_equality_different_matrices () {
        let data_1: Mat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ];

        let data_2: Mat4 = [
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0]
        ];

        let mat4_1: Matrix4 = Matrix4::new(data_1);
        let mat4_2: Matrix4 = Matrix4::new(data_2);

        assert_eq!(mat4_1.equivalent(mat4_2), false);
    }
}