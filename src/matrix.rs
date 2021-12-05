//https://docs.rs/cgmath/0.12.0/cgmath/struct.Matrix4.html

use std::ops;
use crate::equivalent::*;

type ArrayMat2 = [[f64; 2]; 2];
type ArrayMat3 = [[f64; 3]; 3];
type ArrayMat4 = [[f64; 4]; 4];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2 {
    pub data: ArrayMat2
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix3 {
    pub data: ArrayMat3
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4 {
    pub data: ArrayMat4
}

impl Matrix2 {
    pub fn new(data: ArrayMat2) -> Self {
        Matrix2{data}
    }
}

impl Matrix3 {
    pub fn new(data: ArrayMat3) -> Self {
        Matrix3{data}
    }
}

impl Matrix4 {
    pub fn new(data: ArrayMat4) -> Self {
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

impl ops::Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Matrix4) -> Self {
        let mut mat4 = Matrix4::new([[0.0; 4]; 4]);
        for row in 0..4 {
            for colunm in 0..4 {
                mat4.data[row][colunm] = self.data[row][0] * other.data[0][colunm] +
                    self.data[row][1] * other.data[1][colunm] +
                    self.data[row][2] * other.data[2][colunm] +
                    self.data[row][3] * other.data[3][colunm];
            }
        }
        mat4
    }
}

#[cfg(test)]
mod tests_tuple {
    use crate::assert_equivalent;
    use super::*;

    #[test]
    fn constructing_matrix_2_x_2 () {
        let data: ArrayMat2 = [
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
        let data: ArrayMat3 = [
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
        let data: ArrayMat4 = [
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
        let data_1: ArrayMat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ];

        let data_2: ArrayMat4 = [
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
        let data_1: ArrayMat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ];

        let data_2: ArrayMat4 = [
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0]
        ];

        let mat4_1: Matrix4 = Matrix4::new(data_1);
        let mat4_2: Matrix4 = Matrix4::new(data_2);

        assert_eq!(mat4_1.equivalent(mat4_2), false);
        assert!(mat4_1.not_equivalent(mat4_2));
    }

    #[test]
    fn multiply_matrices () {
        let data_1: ArrayMat4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ];

        let data_2: ArrayMat4 = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]
        ];

        let result: ArrayMat4 = [
            [20., 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]
        ];

        let mat4_1: Matrix4 = Matrix4::new(data_1);
        let mat4_2: Matrix4 = Matrix4::new(data_2);
        let mat4_result: Matrix4 = Matrix4::new(result);

        assert_equivalent!(mat4_1 * mat4_2, mat4_result);
    }
}