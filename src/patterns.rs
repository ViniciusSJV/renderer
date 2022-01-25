use crate::color::Color;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::tuple::Tuple;

pub trait Incuse {
    fn color_a(&self) -> Color;
    fn color_b(&self) -> Color;
    fn set_pattern_transform(&mut self, transform: Matrix<4>);
    fn color_at(&self, point: Tuple) -> Color;
    fn color_at_object(&self, object: Object, world_point: Tuple) -> Color;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Patterns {
    Stripe(Stripe),
    Gradient(Gradient)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Stripe {
    color_a: Color,
    color_b: Color,
    transform: Matrix<4>
}

impl Default for Stripe {
    fn default() -> Self {
        Stripe::new(Color::white(), Color::black())
    }
}

impl From<Stripe> for Patterns {
    fn from(stripe: Stripe) -> Self {
        Patterns::Stripe(stripe)
    }
}

impl Stripe {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Stripe { color_a, color_b, transform: Matrix::identity() }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Gradient {
    color_a: Color,
    color_b: Color,
    transform: Matrix<4>
}

impl Default for Gradient {
    fn default() -> Self {
        Gradient::new(Color::white(), Color::black())
    }
}

impl From<Gradient> for Patterns {
    fn from(gradient: Gradient) -> Self {
        Patterns::Gradient(gradient)
    }
}

impl Gradient {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Gradient { color_a, color_b, transform: Matrix::identity() }
    }
}

impl Incuse for Stripe {
    fn color_a(&self) -> Color {
        self.color_a
    }

    fn color_b(&self) -> Color {
        self.color_b
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        let x = point.x;
        if x.floor().abs() as usize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    fn color_at_object(&self, object: Object, world_point: Tuple) -> Color {
        let obj_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform.inverse() * obj_point;
        self.color_at(pattern_point)
    }
}

impl Incuse for Gradient {
    fn color_a(&self) -> Color {
        self.color_a
    }

    fn color_b(&self) -> Color {
        self.color_b
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        todo!()
    }

    fn color_at_object(&self, object: Object, world_point: Tuple) -> Color {
        todo!()
    }
}

impl Incuse for Patterns {
    fn color_a(&self) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_a,
            Patterns::Gradient(ref gradient) => gradient.color_a,
        }
    }

    fn color_b(&self) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_b,
            Patterns::Gradient(ref gradient) => gradient.color_b,
        }
    }

    fn color_at(&self, point: Tuple) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_at(point),
            Patterns::Gradient(ref gradient) => gradient.color_at(point)
        }
    }

    fn color_at_object(&self, object: Object, world_point: Tuple) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_at_object(object, world_point),
            Patterns::Gradient(ref gradient) => gradient.color_at_object(object, world_point)
        }
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        match *self {
            Patterns::Stripe(ref mut stripe) => stripe.set_pattern_transform(transform),
            Patterns::Gradient(ref mut gradient) => gradient.set_pattern_transform(transform),
        }
    }
}

#[cfg(test)]
mod tests_patterns {
    use crate::color::Color;
    use crate::matrix::Matrix;
    use crate::object::{Intersectable, Object};
    use crate::patterns::{Incuse, Patterns, Stripe};
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    fn creating_a_stripe_pattern() {
        let p = Patterns::from(Stripe::new(Color::white(), Color::black()));

        assert_eq!(p.color_a(), Color::white());
        assert_eq!(p.color_b(), Color::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let p = Patterns::from(Stripe::new(Color::white(), Color::black()));
        assert_eq!(p.color_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(p.color_at(Tuple::point(0., 1., 0.)), Color::white());
        assert_eq!(p.color_at(Tuple::point(0., 2., 0.)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let p = Patterns::from(Stripe::new(Color::white(), Color::black()));
        assert_eq!(p.color_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(p.color_at(Tuple::point(0., 0., 1.)), Color::white());
        assert_eq!(p.color_at(Tuple::point(0., 0., 2.)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_x() {
        let p = Patterns::from(Stripe::new(Color::white(), Color::black()));
        assert_eq!(p.color_at(Tuple::point(0., 0., 0.)), Color::white());
        assert_eq!(p.color_at(Tuple::point(0.9, 0., 0.)), Color::white());
        assert_eq!(p.color_at(Tuple::point(1., 0., 0.)), Color::black());

        assert_eq!(p.color_at(Tuple::point(-0.1, 0., 0.)), Color::black());
        assert_eq!(p.color_at(Tuple::point(-1., 0., 0.)), Color::black());
        assert_eq!(p.color_at(Tuple::point(-1.1, 0., 0.)), Color::white());
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut sphere = Sphere::default();
        sphere.set_transform(Matrix::scaling(Tuple::point(2., 2., 2.)));

        let pattern = Patterns::from(Stripe::new(Color::white(), Color::black()));

        let color = pattern.color_at_object(Object::from(sphere), Tuple::point(1.5, 0., 0.));

        assert_eq!(color, Color::white());
    }

    #[test]
    fn stripes_with_an_pattern_transformation() {
        let sphere = Sphere::default();

        let mut pattern = Patterns::from(Stripe::new(Color::white(), Color::black()));
        pattern.set_pattern_transform(Matrix::scaling(Tuple::point(2., 2., 2.)));

        let color = pattern.color_at_object(Object::from(sphere), Tuple::point(1.5, 0., 0.));

        assert_eq!(color, Color::white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut sphere = Sphere::default();
        sphere.set_transform(Matrix::scaling(Tuple::point(2., 2., 2.)));

        let mut pattern = Patterns::from(Stripe::new(Color::white(), Color::black()));
        pattern.set_pattern_transform(Matrix::translation(Tuple::point(0.5, 0., 0.)));

        let color = pattern.color_at_object(Object::from(sphere), Tuple::point(2.5, 0., 0.));

        assert_eq!(color, Color::white());
    }

    #[test]
    fn the_default_pattern_transformation() {
        let p = Stripe::default();
        assert_eq!(p.transform, Matrix::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut p = Stripe::default();
        p.set_pattern_transform(Matrix::translation(Tuple::point(1., 2., 3.)));

        assert_eq!(p.transform, Matrix::translation(Tuple::point(1., 2., 3.)));
    }

}