use crate::color::Color;
use crate::lights::Light;
use crate::object::Object;
use crate::patterns::{Incuse, Patterns};
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub reflactive_index: f64,
    pub pattern: Option<Patterns>
}

impl Material {
    pub fn phong() -> Self {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.,
            transparency: 0.,
            reflactive_index: 1.,
            pattern: None
        }
    }

    pub fn glass() -> Self {
        let mut material = Material::phong();
        material.transparency = 1.;
        material.reflective = 1.;
        material.reflactive_index = 1.5;
        material
    }

    pub fn lighting(&self, object: Object, light: Light, point: Tuple, eye_vector: Tuple, normal_vector: Tuple, in_shadow: bool) -> Color {
        if !point.is_point() || !eye_vector.is_vector() || !normal_vector.is_vector() {
            panic!("Invalid args. point = Tuple::point, eye_vector = Tuple::vector, normal_vector = Tuple::vector")
        }

        let difuse;
        let specular;
        let mut color = self.color;
        if let Some(pattern) = self.pattern {
            color = pattern.color_at_object(object, point);
        }

        let effective_color = color * light.intensity;
        let light_vector = (light.position - point).normalize();

        let ambient_light = effective_color * self.ambient;

        if in_shadow {
            return ambient_light;
        }

        let light_dot_normal = light_vector.dot(normal_vector);
        if light_dot_normal < 0. {
            difuse = Color::black();
            specular = Color::black();
        } else {
            difuse = effective_color * self.diffuse * light_dot_normal;
            let reflect_vector = -light_vector.reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);

            if reflect_dot_eye <= 0. {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powi(self.shininess as i32);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient_light + difuse + specular
    }
}

#[cfg(test)]
mod tests_lights {
    use crate::assert_equivalent;
    use crate::equivalent::Equivalence;
    use crate::lights::Light;
    use crate::patterns::Stripe;
    use crate::sphere::Sphere;
    use super::*;

    #[test]
    pub fn the_default_material() {
        let material = Material::phong();

        assert_equivalent!(material.color, Color::white());
        assert_equivalent!(material.ambient, 0.1);
        assert_equivalent!(material.diffuse, 0.9);
        assert_equivalent!(material.specular, 0.9);
        assert_equivalent!(material.shininess, 200.);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let material = Material::phong();
        let position = Tuple::point(0.,0.,0.);

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_v = Tuple::vector(0., 0., -1.);

        let light = Light::point_light(
            Tuple::point(0., 0., -10.),
            Color::white()
        );

        let result = material.lighting(Object::from(Sphere::default()), light, position, eye_vector, normal_v, false);

        assert_equivalent!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_deg() {
        let material = Material::phong();
        let position = Tuple::point(0.,0.,0.);

        let eye_vector = Tuple::vector(0., (2. as f64).sqrt() / 2., -(2. as f64).sqrt() / 2.);
        let normal_v = Tuple::vector(0., 0., -1.);

        let light = Light::point_light(
            Tuple::point(0., 0., -10.),
            Color::white()
        );

        let result = material.lighting(Object::from(Sphere::default()), light, position, eye_vector, normal_v, false);

        assert_equivalent!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_deg() {
        let material = Material::phong();
        let position = Tuple::point(0.,0.,0.);

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_v = Tuple::vector(0., 0., -1.);

        let light = Light::point_light(
            Tuple::point(0., 10., -10.),
            Color::white()
        );

        let result = material.lighting(Object::from(Sphere::default()), light, position, eye_vector, normal_v, false);

        assert_equivalent!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let material = Material::phong();
        let position = Tuple::point(0.,0.,0.);

        let eye_vector = Tuple::vector(0., -(2. as f64).sqrt() / 2., -(2. as f64).sqrt() / 2.);
        let normal_v = Tuple::vector(0., 0., -1.);

        let light = Light::point_light(
            Tuple::point(0., 10., -10.),
            Color::white()
        );

        let result = material.lighting(Object::from(Sphere::default()), light, position, eye_vector, normal_v, false);

        assert_equivalent!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let material = Material::phong();
        let position = Tuple::point(0.,0.,0.);

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_v = Tuple::vector(0., 0., -1.);

        let light = Light::point_light(
            Tuple::point(0., 0., 10.),
            Color::white()
        );

        let result = material.lighting(Object::from(Sphere::default()), light, position, eye_vector, normal_v, false);

        assert_equivalent!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let material = Material::phong();
        let position = Tuple::point(0.,0.,0.);

        let eye_v =Tuple::vector(0., 0., -1.);
        let normal_v = Tuple::vector(0., 0., -1.);

        let light = Light::point_light(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let in_shadow = true;

        let result = material.lighting(Object::from(Sphere::default()), light, position, eye_v, normal_v, in_shadow);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut material = Material::phong();
        material.pattern = Option::from(Patterns::from(Stripe::new(Color::white(), Color::black())));
        material.ambient = 1.;
        material.diffuse = 0.;
        material.specular = 0.;

        let eye_v = Tuple::vector(0., 0., -1.);
        let normal_v = Tuple::vector(0., 0., -1.);

        let light = Light::point_light(Tuple::point(0., 0., -10.), Color::white());

        let c1 = material.lighting(Object::from(Sphere::default()), light, Tuple::point(0.9, 0., 0.), eye_v, normal_v, false);
        let c2 = material.lighting(Object::from(Sphere::default()), light, Tuple::point(1.1, 0., 0.), eye_v, normal_v, false);

        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let material = Material::phong();
        assert_eq!(material.reflective, 0.);
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let material = Material::phong();
        assert_eq!(material.transparency, 0.);
        assert_eq!(material.reflactive_index, 1.);
    }
}