extern crate renderer;

use renderer::matrix::Matrix;

fn main() {
    let mat4_identity: Matrix<4> = Matrix::identity();

    println!("Mat4 identity: {:?}", mat4_identity);
    println!("Mat4 identity inverse: {:?}", mat4_identity.inverse());

    let mat4: Matrix<4> = Matrix::from([
        [-6., 1., 1., 6.],
        [-8., 5., 8., 6.],
        [-1., 0., 8., 2.],
        [-7., 1., -1., 1.]
    ]);

    println!("Mat4 original: {:?}", mat4);
    println!("Mat4 mult by inverse: {:?}", mat4 * mat4.inverse());
}