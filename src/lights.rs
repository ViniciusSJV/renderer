use crate::{Color, Tuple};

#[derive(Clone, Copy)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color
}

impl Light {
    pub fn point_light(position: Tuple, intensity: Color) -> Self {
        Light { position, intensity }
    }
}

#[cfg(test)]
mod tests_lights {
    use crate::assert_equivalent;
    use crate::equivalent::Equivalence;
    use super::*;

    #[test]
    pub fn an_intersection_encapsulate_t_and_object() {
        let intensity = Color::white();
        let position = Tuple::point(0., 0., 0.);

        let light =  Light::point_light(position, intensity);

        assert_equivalent!(light.position, position);
        assert_equivalent!(light.intensity, intensity);
    }
}