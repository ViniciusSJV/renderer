use crate::color::Color;
use crate::intersection::{Computations, Intersections};
use crate::lights::Light;
use crate::object::{Intersectable, Object};
use crate::ray::Ray;
use crate::tuple::Tuple;

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

    pub fn shade_hit(self, comps: Computations, remaining: u8) -> Color {
        let mut surface = Color::black();
        let shadowed = self.clone().is_shadowed(comps.over_point);
        for &light in self.lights.iter() {
            let color = comps.object.material().lighting(comps.object, light, comps.over_point, comps.eye_v, comps.normal_v, shadowed);
            surface = surface + color;
        }
        let reflected = self.clone().reflected_color(comps, remaining);
        let refracted = self.clone().refracted_color(comps, remaining);
        surface + reflected + refracted
    }

    pub fn reflected_color(self, comps: Computations, remaining: u8) -> Color {
        if comps.object.material().reflective == 0. || remaining <= 0 {
            return Color::black()
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflect_v);
        let color = self.color_at(reflect_ray, remaining - 1);
        color * comps.object.material().reflective
    }

    pub fn refracted_color(self, comps: Computations, remaining: u8) -> Color {
        if comps.object.material().transparency == 0. || remaining <= 0 {
            return Color::black();
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_v.dot(comps.normal_v);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        if sin2_t > 1. {
            return Color::black();
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normal_v * (n_ratio * cos_i - cos_t) - comps.eye_v * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);
        self.color_at(refract_ray, remaining - 1) * comps.object.material().transparency
    }

    pub fn is_shadowed(self, point: Tuple) -> bool {
        for &light in self.lights.iter() {
            let shadow_vector : Tuple = light.position - point;
            let distance = shadow_vector.length();
            let direction = shadow_vector.normalize();

            let shadow_ray  = Ray::new(point, direction);
            let intersections = self.intersect_world(shadow_ray);

            if let Some(hit) = intersections.hit() {
                if hit.t < distance {
                    return true;
                }
            }
        }
        false
    }

    pub fn color_at(self, ray: Ray, remaining: u8) -> Color {
        let xs = self.intersect_world(ray);
        if xs.hit() != None {
            let hit = xs.hit().unwrap();
            let comps = hit.prepare_computations(ray, &xs);
            self.shade_hit(comps, remaining)
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
    use crate::equivalent::Equivalence;
    use crate::intersection::Intersection;
    use crate::lights::Light;
    use crate::materials::Material;
    use crate::matrix::Matrix;
    use crate::patterns::{DefaultPattern, Patterns};
    use crate::plane::Plane;
    use crate::sphere::Sphere;
    use super::*;

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
        let intersection =  Intersection::new(4., s);
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, &xs);
        let color = w.shade_hit(comps, 4);

        assert_equivalent!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = create_default_world();
        w.lights[0] = Light::point_light(Tuple::point(0., 0.25, 0.), Color::new(1., 1., 1.));

        let ray = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = w.objects[1];
        let intersection =  Intersection::new(0.5, s);
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, &xs);
        let color = w.shade_hit(comps, 4);

        assert_equivalent!(color, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_missing() {
        let w = create_default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));
        let color = w.color_at(ray, 4);

        assert_equivalent!(color, Color::new(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = create_default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let color = w.color_at(ray, 4);

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

        let color = w.color_at(ray, 4);

        assert_equivalent!(color, inner.material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = create_default_world();
        let p = Tuple::point(0., 10., 0.);

        assert_eq!(world.is_shadowed(p), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = create_default_world();
        let p = Tuple::point(10., -10., 10.);

        assert!(world.is_shadowed(p));
    }

    #[test]
    fn the_is_no_shadow_when_an_object_is_behind_the_light() {
        let world = create_default_world();
        let p = Tuple::point(-20., 20., -20.);

        assert_eq!(world.is_shadowed(p), false);
    }

    #[test]
    fn the_is_no_shadow_when_an_object_is_behind_the_point() {
        let world = create_default_world();
        let p = Tuple::point(-2., 2., -2.);

        assert_eq!(world.is_shadowed(p), false);
    }

    #[test]
    fn shadow_hit_is_given_an_intersection_in_shadow() {
        let mut world = create_default_world();
        world.lights[0] = Light::point_light(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let s1 = Sphere::default();
        world.objects.push(Object::from(s1));

        let mut s2 = Sphere::default();
        s2.transform = Matrix::translation(Tuple::vector(0., 0., 10.));
        world.objects.push(Object::from(s2));

        let ray = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));

        let intersection = Intersection::new(4., Object::from(s2));
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, &xs);
        let color = world.shade_hit(comps, 4);

        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut world = create_default_world();
        let ray = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let mut shape = Sphere::default();
        shape.material.ambient = 1.;
        world.objects[1] = Object::from(shape);

        let intersection = Intersection::new(1., Object::from(shape));
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, &xs);
        let color = world.reflected_color(comps, 4);

        assert_eq!(color, Color::new(0., 0., 0.));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut world = create_default_world();

        let mut plane = Plane::default();
        plane.material.reflective = 0.5;
        plane.set_transform(Matrix::translation(Tuple::vector(0., -1., 0.)));
        world.objects.push(Object::from(plane));

        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -f64::from(2.).sqrt() / 2., f64::from(2.).sqrt() / 2.));

        let intersection = Intersection::new(f64::from(2.).sqrt(), Object::from(plane));
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, &xs);
        let color = world.reflected_color(comps, 4);

        assert_equivalent!(color, Color::new(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut world = create_default_world();

        let mut plane = Plane::default();
        plane.material.reflective = 0.5;
        plane.set_transform(Matrix::translation(Tuple::vector(0., -1., 0.)));
        world.objects.push(Object::from(plane));

        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -f64::from(2.).sqrt() / 2., f64::from(2.).sqrt() / 2.));

        let intersection = Intersection::new(f64::from(2.).sqrt(), Object::from(plane));
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, &xs);
        let color = world.shade_hit(comps, 4);

        assert_equivalent!(color, Color::new(0.87675, 0.92434, 0.82917));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut world = World::default();
        world.lights.push(Light::point_light(Tuple::point(0., 0., 0.), Color::white()));

        let mut lower = Plane::default();
        lower.material.reflective = 1.;
        lower.set_transform(Matrix::translation(Tuple::vector(0., -1., 0.)));
        world.objects.push(Object::from(lower));

        let mut upper = Plane::default();
        upper.material.reflective = 1.;
        upper.set_transform(Matrix::translation(Tuple::vector(0., 1., 0.)));
        world.objects.push(Object::from(upper));

        let ray = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));

        let color = world.color_at(ray, 4);
        assert_equivalent!(color, Color::new(9.5, 9.5, 9.5));
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut world = create_default_world();

        let mut plane = Plane::default();
        plane.material.reflective = 0.5;
        plane.set_transform(Matrix::translation(Tuple::vector(0., -1., 0.)));
        world.objects.push(Object::from(plane));

        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -f64::from(2.).sqrt() / 2., f64::from(2.).sqrt() / 2.));

        let intersection = Intersection::new(f64::from(2.).sqrt(), Object::from(plane));
        let xs = Intersections::new(vec![intersection]);
        let comps = intersection.prepare_computations(ray, &xs);
        let color = world.reflected_color(comps, 0);

        assert_equivalent!(color, Color::black());
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let world = create_default_world();
        let shape = world.objects[0];

        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let intersect1 = Intersection::new(4.,  shape);
        let intersect2 = Intersection::new(6.,  shape);
        let xs = Intersections::new(vec![intersect1, intersect2]);
        let comp = xs.data[0].prepare_computations(ray, &xs);
        let color = world.refracted_color(comp, 5);

        assert_equivalent!(color, Color::black());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let world = create_default_world();
        let mut shape: Object = world.objects[0];
        let mut material = Material::phong();
        material.transparency = 1.0;
        material.reflactive_index = 1.5;
        shape.set_material(material);

        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let intersect1 = Intersection::new(4.,  shape);
        let intersect2 = Intersection::new(6.,  shape);
        let xs = Intersections::new(vec![intersect1, intersect2]);
        let comp = xs.data[0].prepare_computations(ray, &xs);
        let color = world.refracted_color(comp, 0);

        assert_equivalent!(color, Color::black());
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let world = create_default_world();
        let mut shape: Object = world.objects[0];
        let mut material = Material::phong();
        material.transparency = 1.0;
        material.reflactive_index = 1.5;
        shape.set_material(material);

        let ray = Ray::new(Tuple::point(0., 0., f64::from(2.).sqrt() / 2.), Tuple::vector(0., 1., 0.));

        let intersect1 = Intersection::new(-f64::from(2.).sqrt() / 2.,  shape);
        let intersect2 = Intersection::new(f64::from(2.).sqrt() / 2.,  shape);
        let xs = Intersections::new(vec![intersect1, intersect2]);

        let comp = xs.data[1].prepare_computations(ray, &xs);
        let color = world.refracted_color(comp, 5);

        assert_equivalent!(color, Color::black());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let world = create_default_world();
        let mut a: Object = world.objects[0];
        let mut material_a = a.material();
        material_a.ambient = 1.;
        material_a.pattern = Option::from(Patterns::from(DefaultPattern::default()));
        a.set_material(material_a);

        let mut b: Object = world.objects[0];
        let mut material_b = b.material();
        material_b.transparency = 1.;
        material_b.reflactive_index = 1.5;
        b.set_material(material_b);

        let ray = Ray::new(Tuple::point(0., 0., 0.1), Tuple::vector(0., 1., 0.));

        let intersect1 = Intersection::new(-0.9899,  Object::from(a));
        let intersect2 = Intersection::new(-0.4899,  Object::from(b));
        let intersect3 = Intersection::new(0.4899,  Object::from(b));
        let intersect4 = Intersection::new(0.9899,  Object::from(a));

        let xs = Intersections::new(vec![intersect1, intersect2, intersect3, intersect4]);
        let comp = xs.data[2].prepare_computations(ray, &xs);

        let color = world.refracted_color(comp, 5);
        assert_equivalent!(color, Color::new(0.08, 0.1, 0.06));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut world = create_default_world();
        let mut floor = Plane::default();
        floor.set_transform(Matrix::translation(Tuple::vector(0., -1., 0.)));
        floor.material.transparency = 0.5;
        floor.material.reflactive_index = 1.5;

        world.objects.push(Object::from(floor));

        let mut ball = Sphere::default();
        ball.material.color = Color::new(1., 0., 0.);
        ball.material.ambient = 0.5;
        ball.set_transform(Matrix::translation(Tuple::vector(0., -3.5, -0.5)));

        world.objects.push(Object::from(ball));

        let ray = Ray::new(Tuple::point(0., 0., -3.), Tuple::vector(0., -f64::from(2.).sqrt() / 2., f64::from(2.).sqrt() / 2.));

        let intersect = Intersection::new(f64::from(2.).sqrt(),  Object::from(floor));
        let xs = Intersections::new(vec![intersect]);

        let comp = xs.data[0].prepare_computations(ray, &xs);
        let color = world.shade_hit(comp, 5);
        assert_equivalent!(color, Color::new(0.93642, 0.68642, 0.68642));
    }
}