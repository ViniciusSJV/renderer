use crate::color::Color;
use crate::intersection::{Computations, Intersections};
use crate::lights::Light;
use crate::object::Object;
use crate::ray::Ray;

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Object>, lights: Vec<Light>) -> Self {
        World { objects, lights }
    }

    pub fn intersect_world(&self, ray: Ray) -> Intersections {
        let mut xs = vec![];
        for object in self.objects.iter() {
            xs.extend(object.intersect(ray));
        }
        Intersections::new(xs)
    }

    pub fn shade_hit(self, comps: Computations) -> Color {
        let mut final_color = Color::black();
        for &light in self.lights.iter() {
            let color = comps.object.material().lighting(light, comps.point, comps.eye_v, comps.normal_v);
            final_color = final_color + color;
        }
        final_color
    }

    pub fn color_at(self, ray: Ray) -> Color {
        let xs = self.intersect_world(ray);
        if xs.hit() != None {
            let hit = xs.hit().unwrap();
            let comps = hit.prepare_computations();
            self.shade_hit(comps)
        } else {
            Color::black()
        }
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
    use crate::assert_equivalent;
    use crate::equivalent::*;
    use crate::color::Color;
    use crate::intersection::Intersection;
    use crate::lights::Light;
    use crate::materials::Material;
    use crate::matrix::Matrix;
    use crate::object::Object;
    use crate::ray::Ray;
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

        assert_eq!(world.objects.len(), 2);
        assert_eq!(world.lights.len(), 1);

        assert!(world.objects.contains(&s1));
        assert!(world.objects.contains(&s2));

        assert!(world.lights.contains(&light));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = create_default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let xs = w.intersect_world(ray);

        assert_eq!(xs.data.len(), 4);
        assert_eq!(xs.data[0].t, 4.);
        assert_eq!(xs.data[1].t, 4.5);
        assert_eq!(xs.data[2].t, 5.5);
        assert_eq!(xs.data[3].t, 6.);
    }

    #[test]
    fn shading_an_intersection() {
        let w = create_default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = w.objects[0];
        let i =  Intersection::new(4., s, ray);

        let comps = i.prepare_computations();
        let color = w.shade_hit(comps);

        assert_equivalent!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = create_default_world();
        w.lights[0] = Light::point_light(Tuple::point(0., 0.25, 0.), Color::new(1., 1., 1.));

        let ray = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = w.objects[1];
        let i =  Intersection::new(0.5, s, ray);

        let comps = i.prepare_computations();
        let color = w.shade_hit(comps);

        assert_equivalent!(color, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_missing() {
        let w = create_default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));
        let color = w.color_at(ray);

        assert_equivalent!(color, Color::new(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = create_default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let color = w.color_at(ray);

        assert_equivalent!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = create_default_world();

        let mut outer: Object = w.objects[0];
        let mut outer_material = outer.material();
        outer_material.ambient = 1.;
        outer.set_material(outer_material);

        let mut inner: Object = w.objects[1];
        let mut inner_material = inner.material();
        inner_material.ambient = 1.;
        inner.set_material(inner_material);

        w.objects[0] = outer;
        w.objects[1] = inner;

        let ray = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));

        let color = w.color_at(ray);

        assert_equivalent!(color, inner.material().color);
    }
}