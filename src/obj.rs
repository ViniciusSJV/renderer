use std::fs::read_to_string;

use crate::bounds::Bounds;
use crate::groups::Groups;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::triangle::Triangle;
use crate::tuple::Tuple;

/// Triangles of an OBJ text: `v` lines are vertices, `f` lines are polygons split into a fan
/// from their first vertex. Texture and normal indices (`1/2/3`) are ignored.
pub fn parse_obj(model: &str, material: Material) -> Vec<Object> {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();

    for line in model.lines() {
        let mut fields = line.split_whitespace();
        match fields.next() {
            Some("v") => {
                let x = fields.next().unwrap().parse::<f64>().unwrap();
                let y = fields.next().unwrap().parse::<f64>().unwrap();
                let z = fields.next().unwrap().parse::<f64>().unwrap();
                vertices.push(Tuple::point(x, y, z));
            }
            Some("f") => {
                let indices: Vec<usize> = fields
                    .map(|field| field.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                    .collect();

                for face in indices[1..].windows(2) {
                    let mut triangle = Triangle::new(
                        vertices[indices[0]],
                        vertices[face[0]],
                        vertices[face[1]],
                    );
                    triangle.set_material(material);
                    triangles.push(Object::from(triangle));
                }
            }
            _ => {}
        }
    }

    triangles
}

pub fn load_model(path: &str, material: Material, transform: Matrix<4>) -> Groups {
    let model = read_to_string(path).expect("Error reading model OBJ");
    let mut group = Groups::from(parse_obj(&model, material));
    group.transform = transform;
    group
}

/// Splits the objects in halves along the longest axis of their bounds until each leaf has
/// at most 8 objects.
pub fn build_bvh(mut objects: Vec<Object>) -> Groups {
    if objects.len() <= 8 {
        return Groups::from(objects);
    }

    let mut bounds = Bounds::empty();
    for object in &objects {
        bounds = bounds.union(object.bounds());
    }
    let extent = bounds.maximum - bounds.minimum;
    let axis = if extent.x >= extent.y && extent.x >= extent.z {
        0
    } else if extent.y >= extent.z {
        1
    } else {
        2
    };

    objects.sort_by(|left, right| {
        let a = left.bounds().center();
        let b = right.bounds().center();
        let (a, b) = match axis {
            0 => (a.x, b.x),
            1 => (a.y, b.y),
            _ => (a.z, b.z),
        };
        a.partial_cmp(&b).unwrap()
    });

    let midpoint = objects.len() / 2;
    let right = objects.split_off(midpoint);
    Groups::from_subgroups(vec![build_bvh(objects), build_bvh(right)])
}

/// The model as a BVH that keeps the transform of the loaded group.
pub fn load_bvh(path: &str, material: Material, transform: Matrix<4>) -> Groups {
    let loaded = load_model(path, material, transform);
    let transform = loaded.transform;
    let mut model = build_bvh(loaded.children);
    model.transform = transform;
    model
}

#[cfg(test)]
mod tests_obj {
    use crate::materials::Material;
    use crate::obj::{build_bvh, parse_obj};
    use crate::object::Object;
    use crate::tuple::Tuple;

    #[test]
    fn a_polygon_becomes_a_fan_of_triangles() {
        let model = "v -1 1 0\nv -1 0 0\nv 1 0 0\nv 1 1 0\nv 0 2 0\n\nf 1/1/1 2/2/2 3/3/3 4 5\n";
        let triangles = parse_obj(model, Material::phong());

        assert_eq!(triangles.len(), 3);
        let Object::Triangle(last) = triangles[2] else { panic!("not a triangle") };
        assert_eq!(last.p1, Tuple::point(-1., 1., 0.));
        assert_eq!(last.p2, Tuple::point(1., 1., 0.));
        assert_eq!(last.p3, Tuple::point(0., 2., 0.));
    }

    #[test]
    fn a_bvh_keeps_every_object_in_leaves_of_at_most_eight() {
        let mut model = String::new();
        for i in 0..20 {
            model += &format!("v {i} 0 0\nv {i} 1 0\nv {i}.5 0 1\n");
            model += &format!("f {} {} {}\n", 3 * i + 1, 3 * i + 2, 3 * i + 3);
        }
        let bvh = build_bvh(parse_obj(&model, Material::phong()));

        fn leaves(group: &crate::groups::Groups) -> Vec<usize> {
            if group.subgroups.is_empty() {
                return vec![group.len()];
            }
            group.subgroups.iter().flat_map(leaves).collect()
        }
        let sizes = leaves(&bvh);
        assert_eq!(sizes.iter().sum::<usize>(), 20);
        assert!(sizes.iter().all(|&n| n <= 8));
    }
}
