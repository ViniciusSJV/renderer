//https://docs.rs/cgmath/0.12.0/cgmath/struct.Matrix4.html

use std::ops;
use crate::equivalent::*;
use crate::Tuple;

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

    pub fn determinant(&self) -> f64 {
        let determinant: f64 = (self.data[0][0] * self.data[1][1]) -
            (self.data[0][1] * self.data[1][0]);
        determinant
    }
}

impl Matrix3 {
    pub fn new(data: ArrayMat3) -> Self {
        Matrix3{data}
    }

    pub fn sub_matrix(&self, row: usize, colunm: usize) -> Matrix2 {
        if (row > 2) || (colunm > 2) {
            panic!("Invalid index from Matrix 3. 0 <> 2")
        }

        let mut mat2 = Matrix2::new([[0.0; 2]; 2]);
        let mut mat2_row = 0;
        let mut mat2_column= 0;
        for _row  in 0..3 {
            if _row == row { continue; }
            for _colunm in 0..3 {
                if _colunm == colunm { continue; }
                mat2.data[mat2_row][mat2_column] = self.data[_row][_colunm];
                mat2_column += 1;
                if mat2_column > 1 {
                    mat2_row += 1;
                    mat2_column = 0;
                }
            }
        }
        mat2
    }

    pub fn minor(&self, row: usize, colunm: usize) -> f64{
        self.sub_matrix(row, colunm).determinant()
    }

    pub fn cofactor(&self, row: usize, colunm: usize) -> f64 {
        let minor: f64 = self.minor(row, colunm);
        if (row + colunm) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f64 {
        let determinant: f64 = (self.data[0][0] * self.cofactor(0, 0)) +
            (self.data[0][1] * self.cofactor(0, 1)) +
            (self.data[0][2] * self.cofactor(0, 2));
        determinant
    }
}

impl Matrix4 {
    pub fn new(data: ArrayMat4) -> Self {
        Matrix4{data}
    }

    pub fn identity() -> Self {
        let mat4_identity: Matrix4 = Matrix4::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]
        ]);
        mat4_identity
    }

    pub fn transpose(&self) -> Self {
        let mut mat4: Matrix4 = Matrix4::new([[0.0; 4]; 4]);
        for row in 0..4 {
            for colunm in 0..4 {
                mat4.data[colunm][row] = self.data[row][colunm];
            }
        }
        mat4
    }

    pub fn sub_matrix(&self, row: usize, colunm: usize) -> Matrix3 {
        if (row > 3) || (colunm > 3) {
            panic!("Invalid index from Matrix 4. 0 <> 3")
        }

        let mut mat3 = Matrix3::new([[0.0; 3]; 3]);
        let mut mat3_row= 0;
        let mut mat3_column = 0;
        for _row  in 0..4 {
            if _row == row { continue; }
            for _colunm in 0..4 {
                if _colunm == colunm { continue; }
                mat3.data[mat3_row][mat3_column] = self.data[_row][_colunm];
                mat3_column += 1;
                if mat3_column > 2 {
                    mat3_row += 1;
                    mat3_column = 0;
                }
            }
        }
        mat3
    }

    pub fn minor(&self, row: usize, colunm: usize) -> f64{
        self.sub_matrix(row, colunm).determinant()
    }

    pub fn cofactor(&self, row: usize, colunm: usize) -> f64 {
        let minor: f64 = self.minor(row, colunm);
        if (row + colunm) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f64 {
        let determinant: f64 = (self.data[0][0] * self.cofactor(0, 0)) +
            (self.data[0][1] * self.cofactor(0, 1)) +
            (self.data[0][2] * self.cofactor(0, 2)) +
            (self.data[0][3] * self.cofactor(0, 3));
        determinant
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant().not_equivalent(0.0)
    }

    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!("Its is not invertible")
        }
        let mut mat4_reverse: Matrix4 = Matrix4::new([[0.0; 4];4]);
        let determinant = self.determinant();
        for row  in 0..4 {
            for colunm in 0..4 {
                mat4_reverse.data[colunm][row] = self.cofactor(row, colunm) / determinant;
            }
        }
        mat4_reverse
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
        let mut mat4: Matrix4 = Matrix4::new([[0.0; 4]; 4]);
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

impl ops::Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        Tuple {
            x: self.data[0][0] * tuple.x + self.data[0][1] * tuple.y + self.data[0][2] * tuple.z + self.data[0][3] * tuple.w,
            y: self.data[1][0] * tuple.x + self.data[1][1] * tuple.y + self.data[1][2] * tuple.z + self.data[1][3] * tuple.w,
            z: self.data[2][0] * tuple.x + self.data[2][1] * tuple.y + self.data[2][2] * tuple.z + self.data[2][3] * tuple.w,
            w: self.data[3][0] * tuple.x + self.data[3][1] * tuple.y + self.data[3][2] * tuple.z + self.data[3][3] * tuple.w,
        }
    }
}

#[cfg(test)]
mod tests_matrix {
    use crate::assert_equivalent;
    use super::*;

    #[test]
    fn constructing_mat2 () {
        let mat2: Matrix2 = Matrix2::new([
            [-3.0, 5.0],
            [1.0, -2.0]
        ]);

        assert_eq!(mat2.data[0][0], -3.0);
        assert_eq!(mat2.data[0][1], 5.0);
        assert_eq!(mat2.data[1][0], 1.0);
        assert_eq!(mat2.data[1][1], -2.0);
    }

    #[test]
    fn constructing_mat3 () {
        let mat3: Matrix3 = Matrix3::new([
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]
        ]);

        assert_eq!(mat3.data[0][0], -3.0);
        assert_eq!(mat3.data[1][1], -2.0);
        assert_eq!(mat3.data[2][2], 1.0);
    }

    #[test]
    fn constructing_mat4 () {
        let mat4: Matrix4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]
        ]);

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
        let mat4_1: Matrix4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ]);
        let mat4_2: Matrix4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ]);

        assert_eq!(mat4_1.equivalent(mat4_2), true);
        assert!(mat4_1.equivalent(mat4_2));
    }

    #[test]
    fn matrix_equality_different_matrices () {
        let mat4_1: Matrix4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ]);
        let mat4_2: Matrix4 = Matrix4::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0]
        ]);

        assert_eq!(mat4_1.equivalent(mat4_2), false);
        assert!(mat4_1.not_equivalent(mat4_2));
    }

    #[test]
    fn multiply_matrices () {
        let mat4_1: Matrix4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ]);
        let mat4_2: Matrix4 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]
        ]);
        let mat4_result: Matrix4 = Matrix4::new([
            [20., 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]
        ]);

        assert_equivalent!(mat4_1 * mat4_2, mat4_result);
    }

    #[test]
    fn multiply_matrix_by_a_tuple () {
        let mat4: Matrix4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);
        let tuple: Tuple = Tuple::new(1., 2., 3., 1.);

        let result: Tuple = Tuple::new(18., 24., 33., 1.);

        assert_equivalent!(mat4 * tuple, result);
    }

    #[test]
    fn multiply_a_matrix_by_the_identity_matrix() {
        let mat4: Matrix4 = Matrix4::new([
            [0.0, 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.]
        ]);

        let identity: Matrix4 = Matrix4::identity();

        assert_equivalent!(mat4 * identity, mat4);
    }

    #[test]
    fn multiply_a_matrix_identity_the_tuple() {
        let tuple:Tuple = Tuple::new(1., 2., 3., 4.);

        let identity: Matrix4 = Matrix4::identity();

        assert_equivalent!(identity * tuple, tuple);
    }

    #[test]
    fn transposing_a_matrix() {
        let mat4: Matrix4 = Matrix4::new([
            [0.0, 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.]
        ]);

        let result: Matrix4 = mat4.transpose();

        let expected_result: Matrix4 = Matrix4::new([
            [0.0, 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0.,8. , 3., 8.]
        ]);

        assert_equivalent!(result, expected_result);
    }

    #[test]
    fn transpose_identity_matrix() {
        let identity: Matrix4 = Matrix4::identity();

        assert_equivalent!(identity, identity.transpose());
    }

    #[test]
    fn calculating_the_determinant_of_a_mat2() {
        let mat2: Matrix2 = Matrix2::new([
            [1., 5.],
            [-3., 2.]
        ]);

        assert_equivalent!(mat2.determinant(), 17.);
    }

    #[test]
    fn submatrix_of_mat3_is_a_mat2() {
        let mat3: Matrix3 = Matrix3::new([
            [1.,5.,0.],
            [-3., 2., 7.],
            [0., 6., -3.]
        ]);

        let result: Matrix2 = mat3.sub_matrix(0, 2);

        let expected_result: Matrix2 = Matrix2::new([
            [-3., 2.],
            [0., 6.]
        ]);

        assert_equivalent!(result, expected_result)
    }

    #[test]
    fn submatrix_of_mat4_is_a_mat3() {
        let mat4: Matrix4 = Matrix4::new([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.]
        ]);

        let result: Matrix3 = mat4.sub_matrix(2, 1);

        let expected_result: Matrix3 = Matrix3::new([
            [-6., 1., 6.],
            [-8., 8., 6.],
            [-7., -1., 1.]
        ]);

        assert_equivalent!(result, expected_result)
    }

    #[test]
    fn calculating_a_minor_of_mat3() {
        let mat3 = Matrix3::new([
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        ]);

        let mat2 = mat3.sub_matrix(1, 0);

        assert_equivalent!(mat2.determinant(), 25.);
        assert_equivalent!(mat3.minor(1, 0), 25.);
    }

    #[test]
    fn calculating_a_cofactor_of_a_mat3() {
        let mat3: Matrix3 = Matrix3::new([
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        ]);

        assert_equivalent!(mat3.minor(0, 0), -12.);
        assert_equivalent!(mat3.cofactor(0, 0), -12.);
        assert_equivalent!(mat3.minor(1, 0), 25.);
        assert_equivalent!(mat3.cofactor(1, 0), -25.);
    }

    #[test]
    fn calculating_the_determinant_of_mat3() {
        let mat3: Matrix3 = Matrix3::new([
            [1., 2., 6.],
            [-5., 8., -4.],
            [2., 6., 4.]
        ]);

        assert_equivalent!(mat3.cofactor(0, 0), 56.);
        assert_equivalent!(mat3.cofactor(0, 1), 12.);
        assert_equivalent!(mat3.cofactor(0, 2), -46.);
        assert_equivalent!(mat3.determinant(), -196.)
    }

    #[test]
    fn calculating_the_determinant_of_mat4() {
        let mat4: Matrix4 = Matrix4::new([
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.]
        ]);

        assert_equivalent!(mat4.cofactor(0, 0), 690.);
        assert_equivalent!(mat4.cofactor(0, 1), 447.);
        assert_equivalent!(mat4.cofactor(0, 2), 210.);
        assert_equivalent!(mat4.cofactor(0, 3), 51.);
        assert_equivalent!(mat4.determinant(), -4071.)
    }

    #[test]
    fn is_invertible_mat4() {
        let mat4: Matrix4 = Matrix4::new([
            [6., 4., 4., 4.],
            [5., 5., 7., 6.],
            [4., -9., 3., -7.],
            [9., 1., 7., -6.]
        ]);

        assert_equivalent!(mat4.determinant(), -2120.);
        assert!(mat4.is_invertible());
    }

    #[test]
    fn is_not_invertible_mat4() {
        let mat4: Matrix4 = Matrix4::new([
            [-4., 2., -2., -3.],
            [9., 6., 2., 6.],
            [0., -5., 1., -5.],
            [0., 0., 0., 0.]
        ]);

        assert_equivalent!(mat4.determinant(), 0.);
        assert!(!mat4.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_mat4() {
        let mat4: Matrix4 = Matrix4::new([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.]
        ]);

        let mat4_inverse = mat4.inverse();
        let mat4_inverse_result: Matrix4 = Matrix4::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639]
        ]);

        assert_equivalent!(mat4.determinant(), 532.);
        assert_equivalent!(mat4.cofactor(2, 3), -160.);
        assert_equivalent!(mat4_inverse.data[3][2], -160./532.);
        assert_equivalent!(mat4.cofactor(3, 2), 105.);
        assert_equivalent!(mat4_inverse.data[2][3], 105./532.);
        assert_equivalent!(mat4_inverse, mat4_inverse_result);
    }

    #[test]
    fn calculating_the_inverse_of_another_mat4() {
        let mat4: Matrix4 = Matrix4::new([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.]
        ]);

        let mat4_inverse = mat4.inverse();
        let mat4_inverse_result: Matrix4 = Matrix4::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308]
        ]);

        assert_equivalent!(mat4_inverse, mat4_inverse_result);
    }

    #[test]
    fn calculating_the_inverse_of_third_mat4() {
        let mat4: Matrix4 = Matrix4::new([
            [9., 3., 0., 9.],
            [-5., -2., -6., -3.],
            [-4., 9., 6., 4.],
            [-7., 6., 6., 2.]
        ]);

        let mat4_inverse = mat4.inverse();
        let mat4_inverse_result: Matrix4 = Matrix4::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333]
        ]);

        assert_equivalent!(mat4_inverse, mat4_inverse_result);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a: Matrix4 = Matrix4::new([
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-7., 6., 6., 2.]
        ]);

        let b: Matrix4 = Matrix4::new([
            [8., 2., 2., 2.],
            [3., -1., 7., 0.],
            [7., 0., 5., 4.],
            [6., -2., 0., 5.]
        ]);

        let c: Matrix4 = a * b;

        assert_equivalent!(c * b.inverse(), a);
    }
}