use crate::EPSILON;
use crate::intersection::{Intersection, Intersections};
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,
    pub normal: Tuple,
    pub material: Material,
    pub transform: Matrix<4>,
    /// Normais por vertice (smooth triangle): a normal num ponto e a interpolacao das tres.
    pub normals: Option<[Tuple; 3]>,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(e1).normalize();
        Triangle { p1, p2, p3, e1, e2, normal, material: Material::phong(), transform: Matrix::identity(), normals: None }
    }

    /// Um triangulo com uma normal em cada vertice: a superficie parece curva.
    pub fn smooth(p1: Tuple, p2: Tuple, p3: Tuple, n1: Tuple, n2: Tuple, n3: Tuple) -> Self {
        Triangle { normals: Some([n1, n2, n3]), ..Triangle::new(p1, p2, p3) }
    }
}

impl Intersectable for Triangle {
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        crate::count!();
        let dir_cross_e2 = local_ray.direction.cross(self.e2);
        let det = self.e1.dot(dir_cross_e2);

        if det.abs() < EPSILON {
            return  Intersections::new(vec![]);
        }

        let f = 1.0 / det;

        let p1_to_origin = local_ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_e2);

        if u < 0. || u > 1. {
            return  Intersections::new(vec![]);
        }

        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * local_ray.direction.dot(origin_cross_e1);

        if v < 0. || (u + v) > 1. {
            return  Intersections::new(vec![]);
        }

        let t = f * self.e2.dot(origin_cross_e1);

        Intersections::new(vec![
            Intersection::new(t, Object::from(*self))
        ])
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let Some([n1, n2, n3]) = self.normals else { return self.normal };
        // Coordenadas baricentricas do ponto: o peso de cada vertice.
        let v0 = local_point - self.p1;
        let (d00, d01, d11) = (self.e1.dot(self.e1), self.e1.dot(self.e2), self.e2.dot(self.e2));
        let (d20, d21) = (v0.dot(self.e1), v0.dot(self.e2));
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let mut n = n1 * (1. - v - w) + n2 * v + n3 * w;
        n.w = 0.;
        n.normalize()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_material(&mut self, material: Material) {
        self.material = material
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform
    }
}

#[cfg(test)]
mod tests_triangle {
    use crate::assert_equivalent;
    use crate::equivalent::Equivalence;
    use crate::object::Intersectable;
    use crate::ray::Ray;
    use crate::triangle::Triangle;
    use crate::tuple::Tuple;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);

        let t = Triangle::new(p1, p2, p3);

        assert_equivalent!(t.p1, p1);
        assert_equivalent!(t.p2, p2);
        assert_equivalent!(t.p3, p3);
        assert_equivalent!(t.e1, Tuple::vector(-1., -1., 0.));
        assert_equivalent!(t.e2, Tuple::vector(1., -1., 0.));
        assert_equivalent!(t.normal, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);

        let t = Triangle::new(p1, p2, p3);

        let n1 = t.normal_at(Tuple::point(0., 0.5, 0.));
        let n2 = t.normal_at(Tuple::point(-0.5, 0.75, 0.));
        let n3 = t.normal_at(Tuple::point(0.5, 0.25, 0.));

        assert_equivalent!(t.normal, n1);
        assert_equivalent!(t.normal, n2);
        assert_equivalent!(t.normal, n3);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);

        let t = Triangle::new(p1, p2, p3);

        let ray = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 1., 0.));

        let xs = t.local_intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_and_p3_edge() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);

        let t = Triangle::new(p1, p2, p3);

        let ray = Ray::new(Tuple::point(1., 1., -2.), Tuple::vector(0., 0., 1.));

        let xs = t.local_intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_and_p2_edge() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);

        let t = Triangle::new(p1, p2, p3);

        let ray = Ray::new(Tuple::point(-1., 1., -2.), Tuple::vector(0., 0., 1.));

        let xs = t.local_intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p2_and_p3_edge() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);

        let t = Triangle::new(p1, p2, p3);

        let ray = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 0., 1.));

        let xs = t.local_intersect(ray);

        assert_eq!(xs.data.len(), 0);
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);

        let t = Triangle::new(p1, p2, p3);

        let ray = Ray::new(Tuple::point(0., 0.5, -2.), Tuple::vector(0., 0., 1.));

        let xs = t.local_intersect(ray);

        assert_eq!(xs.data.len(), 1);
        assert_eq!(xs.data[0].t, 2.);
    }
}