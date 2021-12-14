use crate::lights::Light;
use crate::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Object>, lights: Vec<Light>) -> Self {
        World { objects, lights }
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            objects: vec![],
            lights: vec![],
        }
    }
}

#[cfg(test)]
mod tests_world {
    use crate::color::Color;
    use crate::lights::Light;
    use crate::materials::Material;
    use crate::matrix::Matrix;
    use crate::object::Object;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use crate::world::World;

    fn create_default_world() -> World {
        let light = Light::point_light(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut material = Material::phong();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;

        let mut sphere = Sphere::default();
        sphere.set_material(material);

        let mut sphere_2 = Sphere::default();
        sphere_2.set_transform(Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5)));

        let s1 = Object::from(sphere);
        let s2 = Object::from(sphere_2);

        World::new(vec![s1, s2], vec![light])
    }

    #[test]
    fn creating_world() {
        let world = World::default();

        assert_eq!(world.objects, vec![]);
        assert_eq!(world.lights, vec![]);
    }

    #[test]
    fn the_default_world() {
        let light = Light::point_light(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut material = Material::phong();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;

        let mut sphere = Sphere::default();
        sphere.set_material(material);

        let mut sphere_2 = Sphere::default();
        sphere_2.set_transform(Matrix::scaling(Tuple::vector(0.5, 0.5, 0.5)));

        let s1 = Object::from(sphere);
        let s2 = Object::from(sphere_2);

        let world = create_default_world();

        assert_eq!(2, world.objects.len());
        assert_eq!(1, world.lights.len());

        assert!(world.objects.contains(&s1));
        assert!(world.objects.contains(&s2));

        assert!(world.lights.contains(&light));
    }
}