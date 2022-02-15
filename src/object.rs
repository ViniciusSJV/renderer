use crate::cone::Cone;
use crate::intersection::Intersections;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::plane::Plane;
use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::triangle::Triangle;
use crate::tuple::Tuple;

pub trait Intersectable {
    fn local_intersect(&self, local_ray: Ray) -> Intersections;
    fn local_normal_at(&self, world_point: Tuple) -> Tuple;
    fn material(&self) -> Material;
    fn transform(&self) -> Matrix<4>;
    fn set_material(&mut self, material: Material);
    fn set_transform(&mut self, transform: Matrix<4>);

    fn intersect(&self, original_ray: Ray) -> Intersections {
        let local_ray = original_ray.set_transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let local_point = self.transform().inverse() * point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.transform().inverse().transpose() * local_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Triangle(Triangle)
}

impl From<Sphere> for Object {
    fn from(sphere: Sphere) -> Self {
        Object::Sphere(sphere)
    }
}

impl From<Plane> for Object {
    fn from(plane: Plane) -> Self {
        Object::Plane(plane)
    }
}

impl From<Cube> for Object {
    fn from(cube: Cube) -> Self {
        Object::Cube(cube)
    }
}

impl From<Cylinder> for Object {
    fn from(cylinder: Cylinder) -> Self {
        Object::Cylinder(cylinder)
    }
}

impl From<Cone> for Object {
    fn from(cone: Cone) -> Self {
        Object::Cone(cone)
    }
}

impl From<Triangle> for Object {
    fn from(triangle: Triangle) -> Self {
        Object::Triangle(triangle)
    }
}

impl Intersectable for Object {
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        match *self {
            Object::Sphere(ref sphere) => sphere.local_intersect(local_ray),
            Object::Plane(ref plane) => plane.local_intersect(local_ray),
            Object::Cube(ref cube) => cube.local_intersect(local_ray),
            Object::Cylinder(ref cylinder) => cylinder.local_intersect(local_ray),
            Object::Cone(ref cone) => cone.local_intersect(local_ray),
            Object::Triangle(ref triangle) => triangle.local_intersect(local_ray),
        }
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        match *self {
            Object::Sphere(ref sphere) => sphere.local_normal_at(point),
            Object::Plane(ref plane) => plane.local_normal_at(point),
            Object::Cube(ref cube) => cube.local_normal_at(point),
            Object::Cylinder(ref cylinder) => cylinder.local_normal_at(point),
            Object::Cone(ref cone) => cone.local_normal_at(point),
            Object::Triangle(ref triangle) => triangle.local_normal_at(point),
        }
    }

    fn material(&self) -> Material {
        match *self {
            Object::Sphere(ref sphere) => sphere.material,
            Object::Plane(ref plane) => plane.material,
            Object::Cube(ref cube) => cube.material,
            Object::Cylinder(ref cylinder) => cylinder.material,
            Object::Cone(ref cone) => cone.material,
            Object::Triangle(ref triangle) => triangle.material,
        }
    }

    fn transform(&self) -> Matrix<4> {
        match *self {
            Object::Sphere(ref sphere) => sphere.transform,
            Object::Plane(ref plane) => plane.transform,
            Object::Cube(ref cube) => cube.transform,
            Object::Cylinder(ref cylinder) => cylinder.transform,
            Object::Cone(ref cone) => cone.transform,
            Object::Triangle(ref triangle) => triangle.transform,
        }
    }

    fn set_material(&mut self, material: Material) {
        match *self {
            Object::Sphere(ref mut sphere) => sphere.material = material,
            Object::Plane(ref mut plane) => plane.material = material,
            Object::Cube(ref mut cube) => cube.material = material,
            Object::Cylinder(ref mut cylinder) => cylinder.material = material,
            Object::Cone(ref mut cone) => cone.material = material,
            Object::Triangle(ref mut triangle) => triangle.material = material,
        }
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        match *self {
            Object::Sphere(ref mut sphere) => sphere.transform = transform,
            Object::Plane(ref mut plane) => plane.transform = transform,
            Object::Cube(ref mut cube) => cube.transform = transform,
            Object::Cylinder(ref mut cylinder) => cylinder.transform = transform,
            Object::Cone(ref mut cone) => cone.transform = transform,
            Object::Triangle(ref mut triangle) => triangle.transform = transform,
        }
    }
}

#[cfg(test)]
mod tests_object {
    use crate::intersection::Intersection;
    use crate::object::Object;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    pub fn an_intersection_encapsulate_t_and_object() {
        let sphere = Sphere::new(Tuple::point(0., 0., 0.), 1.);
        let object = Object::from(sphere);

        let intersect = Intersection::new(3.5, object);

        assert_eq!(intersect.t, 3.5);
        assert_eq!(intersect.object, Object::from(sphere));
    }
}