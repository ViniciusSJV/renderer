use crate::color::Color;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::tuple::Tuple;

pub trait Incuse {
    fn color_a(&self) -> Color;
    fn color_b(&self) -> Color;
    fn transform(&self) -> Matrix<4>;
    fn set_pattern_transform(&mut self, transform: Matrix<4>);

    fn color_at_object(&self, object: Object, world_point: Tuple) -> Color {
        let obj_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform().inverse() * obj_point;
        self.color_at(pattern_point)
    }

    fn color_at(&self, point: Tuple) -> Color;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Patterns {
    Stripe(Stripe),
    LinearGradient(LinearGradient),
    Ring(Ring),
    Checkers(Checkers)
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
pub struct LinearGradient {
    color_a: Color,
    color_b: Color,
    transform: Matrix<4>
}

impl Default for LinearGradient {
    fn default() -> Self {
        LinearGradient::new(Color::white(), Color::black())
    }
}

impl From<LinearGradient> for Patterns {
    fn from(linear_gradient: LinearGradient) -> Self {
        Patterns::LinearGradient(linear_gradient)
    }
}

impl LinearGradient {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        LinearGradient { color_a, color_b, transform: Matrix::identity() }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ring {
    color_a: Color,
    color_b: Color,
    transform: Matrix<4>
}

impl Default for Ring {
    fn default() -> Self {
        Ring::new(Color::white(), Color::black())
    }
}

impl From<Ring> for Patterns {
    fn from(ring: Ring) -> Self {
        Patterns::Ring(ring)
    }
}

impl Ring {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Ring { color_a, color_b, transform: Matrix::identity() }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Checkers {
    color_a: Color,
    color_b: Color,
    transform: Matrix<4>
}

impl Default for Checkers {
    fn default() -> Self {
        Checkers::new(Color::white(), Color::black())
    }
}

impl From<Checkers> for Patterns {
    fn from(checkers: Checkers) -> Self {
        Patterns::Checkers(checkers)
    }
}

impl Checkers {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Checkers { color_a, color_b, transform: Matrix::identity() }
    }
}

impl Incuse for Stripe {
    fn color_a(&self) -> Color {
        self.color_a
    }

    fn color_b(&self) -> Color {
        self.color_b
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        let x = point.x;
        if x.floor().abs() % 2. == 0. {
            self.color_a
        } else {
            self.color_b
        }
    }
}

impl Incuse for LinearGradient {
    fn color_a(&self) -> Color {
        self.color_a
    }

    fn color_b(&self) -> Color {
        self.color_b
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        self.color_a + ((self.color_b - self.color_a) * point.x)

        // another way
        //let distance = self.color_b - self.color_a;
        //let fraction = (point.x + 1.) * 0.5;
        //self.color_a + distance * fraction
    }
}

impl Incuse for Ring {
    fn color_a(&self) -> Color {
        self.color_a
    }

    fn color_b(&self) -> Color {
        self.color_b
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2. == 0. {
            self.color_a
        } else {
            self.color_b
        }
    }
}

impl Incuse for Checkers {
    fn color_a(&self) -> Color {
        self.color_a
    }

    fn color_b(&self) -> Color {
        self.color_b
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
            self.color_a
        } else {
            self.color_b
        }
    }
}

impl Incuse for Patterns {
    fn color_a(&self) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_a,
            Patterns::LinearGradient(ref linear_gradient) => linear_gradient.color_a,
            Patterns::Ring(ref ring) => ring.color_a,
            Patterns::Checkers(ref checkers) => checkers.color_a,
        }
    }

    fn color_b(&self) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_b,
            Patterns::LinearGradient(ref linear_gradient) => linear_gradient.color_b,
            Patterns::Ring(ref ring) => ring.color_b,
            Patterns::Checkers(ref checkers) => checkers.color_b,
        }
    }

    fn transform(&self) -> Matrix<4> {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.transform,
            Patterns::LinearGradient(ref linear_gradient) => linear_gradient.transform,
            Patterns::Ring(ref ring) => ring.transform,
            Patterns::Checkers(ref checkers) => checkers.transform,
        }
    }

    fn set_pattern_transform(&mut self, transform: Matrix<4>) {
        match *self {
            Patterns::Stripe(ref mut stripe) => stripe.set_pattern_transform(transform),
            Patterns::LinearGradient(ref mut linear_gradient) => linear_gradient.set_pattern_transform(transform),
            Patterns::Ring(ref mut ring) => ring.set_pattern_transform(transform),
            Patterns::Checkers(ref mut checkers) => checkers.set_pattern_transform(transform),
        }
    }

    fn color_at_object(&self, object: Object, world_point: Tuple) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_at_object(object, world_point),
            Patterns::LinearGradient(ref linear_gradient) => linear_gradient.color_at_object(object, world_point),
            Patterns::Ring(ref ring) => ring.color_at_object(object, world_point),
            Patterns::Checkers(ref checkers) => checkers.color_at_object(object, world_point),
        }
    }

    fn color_at(&self, point: Tuple) -> Color {
        match *self {
            Patterns::Stripe(ref stripe) => stripe.color_at(point),
            Patterns::LinearGradient(ref linear_gradient) => linear_gradient.color_at(point),
            Patterns::Ring(ref ring) => ring.color_at(point),
            Patterns::Checkers(ref checkers) => checkers.color_at(point),
        }
    }
}

#[cfg(test)]
mod tests_patterns {
    use crate::color::Color;
    use crate::matrix::Matrix;
    use crate::object::{Intersectable, Object};
    use crate::patterns::{Checkers, LinearGradient, Incuse, Patterns, Ring, Stripe};
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

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Patterns::from(LinearGradient::new(Color::white(), Color::black()));

        let color1 = pattern.color_at(Tuple::point(0.,0., 0.));
        let color2 = pattern.color_at(Tuple::point(0.25,0., 0.));
        let color3 = pattern.color_at(Tuple::point(0.5,0., 0.));
        let color4 = pattern.color_at(Tuple::point(0.75,0., 0.));

        assert_eq!(color1, Color::white());
        assert_eq!(color2, Color::new(0.75,0.75,0.75));
        assert_eq!(color3, Color::new(0.5,0.5,0.5));
        assert_eq!(color4, Color::new(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Patterns::from(Ring::new(Color::white(), Color::black()));

        let color1 = pattern.color_at(Tuple::point(0.,0., 0.));
        let color2 = pattern.color_at(Tuple::point(1.,0., 0.));
        let color3 = pattern.color_at(Tuple::point(0.,0., 1.));
        let color4 = pattern.color_at(Tuple::point(0.708,0., 0.708));

        assert_eq!(color1, Color::white());
        assert_eq!(color2, Color::black());
        assert_eq!(color3, Color::black());
        assert_eq!(color4, Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Patterns::from(Checkers::new(Color::white(), Color::black()));

        let color1 = pattern.color_at(Tuple::point(0.,0., 0.));
        let color2 = pattern.color_at(Tuple::point(0.99,0., 0.));
        let color3 = pattern.color_at(Tuple::point(1.01,0., 0.));

        assert_eq!(color1, Color::white());
        assert_eq!(color2, Color::white());
        assert_eq!(color3, Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Patterns::from(Checkers::new(Color::white(), Color::black()));

        let color1 = pattern.color_at(Tuple::point(0.,0., 0.));
        let color2 = pattern.color_at(Tuple::point(0.,0.99, 0.));
        let color3 = pattern.color_at(Tuple::point(0.,1.01, 0.));

        assert_eq!(color1, Color::white());
        assert_eq!(color2, Color::white());
        assert_eq!(color3, Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Patterns::from(Checkers::new(Color::white(), Color::black()));

        let color1 = pattern.color_at(Tuple::point(0.,0., 0.));
        let color2 = pattern.color_at(Tuple::point(0.,0., 0.99));
        let color3 = pattern.color_at(Tuple::point(0.,0., 1.01));

        assert_eq!(color1, Color::white());
        assert_eq!(color2, Color::white());
        assert_eq!(color3, Color::black());
    }

}