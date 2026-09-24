use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub trait Transform {
    fn view_transform(self, to: Tuple, up: Tuple) -> Matrix<4>;
}

impl Transform for Tuple {
    fn view_transform(self, to: Tuple, up: Tuple) -> Matrix<4> {
        let forward = (to - self).normalize();
        let upn = up.normalize();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);

        let orientation = Matrix::from([
            [left.x, left.y, left.z, 0.],
            [true_up.x, true_up.y, true_up.z, 0.],
            [-forward.x, -forward.x, -forward.z, 0.],
            [0., 0., 0., 1.]
        ]);

        orientation * Matrix::translation(Tuple::vector(-self.x, -self.y, -self.z))
    }
}

#[cfg(test)]
mod tests_transformations {
    use crate::assert_equivalent;
    use crate::equivalent::*;
    use crate::matrix::Matrix;
    use crate::transformations::Transform;
    use crate::tuple::Tuple;

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., -1.);
        let up = Tuple::vector(0., 1., 0.);

        let t = from.view_transform(to, up);

        assert_equivalent!(t, Matrix::identity());
    }

    #[test]
    fn the_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., 1.);
        let up = Tuple::vector(0., 1., 0.);

        let t = from.view_transform(to, up);

        assert_equivalent!(t, Matrix::scaling(Tuple::vector(-1., 1., -1.)));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Tuple::point(0., 0., 8.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);

        let t = from.view_transform(to, up);

        assert_equivalent!(t, Matrix::translation(Tuple::vector(0., 0., -8.)));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Tuple::point(1., 3., 2.);
        let to = Tuple::point(4., -2., 8.);
        let up = Tuple::vector(1., 1., 0.);

        let t = from.view_transform(to, up);

        let mat4: Matrix<4> = Matrix::from([
            [-0.5070925528371099, 0.5070925528371099, 0.6761234037828132, -2.366431913239846],
            [0.7677159338596801, 0.6060915267313263, 0.12121830534626524, -2.8284271247461894],
            [-0.35856858280031806, -0.35856858280031806, -0.7171371656006361, 2.8685486624025445],
            [0.0, 0.0, 0.0, 1.0]
        ]);

        assert_equivalent!(t, mat4);
    }
}