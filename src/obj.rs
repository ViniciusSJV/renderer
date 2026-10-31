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

/// The same triangles as `parse_obj`, smooth: each vertex gets a normal, the one written in
/// the file (`vn`, referenced as `v//vn` or `v/vt/vn`) or, without it, the area-weighted mean of
/// the faces around the vertex. Shading interpolates the three normals inside each triangle.
///
/// The vertices and normals are taken to `transform` here, so each triangle is already where it
/// is drawn: a triangle in a group does not see the group's transform when shading, and a
/// normal in the wrong space shades the surface wrong.
pub fn parse_obj_smooth(model: &str, material: Material, transform: Matrix<4>) -> Vec<Object> {
    let normal_transform = transform.inverse().transpose();
    let mut vertices = Vec::new();
    let mut written = Vec::new();
    let mut faces: Vec<Vec<(usize, Option<usize>)>> = Vec::new();
    for line in model.lines() {
        let mut fields = line.split_whitespace();
        let xyz = |f: &mut std::str::SplitWhitespace| {
            let mut next = || f.next().unwrap().parse::<f64>().unwrap();
            (next(), next(), next())
        };
        match fields.next() {
            Some("v") => {
                let (x, y, z) = xyz(&mut fields);
                vertices.push(transform * Tuple::point(x, y, z));
            }
            Some("vn") => {
                let (x, y, z) = xyz(&mut fields);
                let mut n = normal_transform * Tuple::vector(x, y, z);
                n.w = 0.;
                written.push(n.normalize());
            }
            Some("f") => faces.push(
                fields
                    .map(|field| {
                        let mut parts = field.split('/');
                        let v = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                        let n = parts.nth(1).and_then(|n| n.parse::<usize>().ok()).map(|n| n - 1);
                        (v, n)
                    })
                    .collect(),
            ),
            _ => {}
        }
    }
    let fans = |face: &Vec<(usize, Option<usize>)>| (1..face.len().saturating_sub(1)).map(|i| [face[0], face[i], face[i + 1]]).collect::<Vec<_>>();
    // Each face normal, unnormalized (its length is twice the area), added to its vertices.
    let mut mean = vec![Tuple::vector(0., 0., 0.); vertices.len()];
    for face in &faces {
        for [a, b, c] in fans(face) {
            let (pa, pb, pc) = (vertices[a.0], vertices[b.0], vertices[c.0]);
            let n = (pc - pa).cross(pb - pa);
            for v in [a.0, b.0, c.0] {
                mean[v] = mean[v] + n;
            }
        }
    }
    let normal = |(v, n): (usize, Option<usize>)| n.and_then(|n| written.get(n).copied()).unwrap_or_else(|| mean[v].normalize());
    let mut triangles = Vec::new();
    for face in &faces {
        for [a, b, c] in fans(face) {
            let mut triangle = Triangle::smooth(vertices[a.0], vertices[b.0], vertices[c.0], normal(a), normal(b), normal(c));
            triangle.set_material(material);
            triangles.push(Object::from(triangle));
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

/// `load_bvh` with smooth triangles (see `parse_obj_smooth`).
pub fn load_bvh_smooth(path: &str, material: Material, transform: Matrix<4>) -> Groups {
    let model = read_to_string(path).expect("Error reading model OBJ");
    // The transform is already in the vertices: the group stays at the identity.
    build_bvh(parse_obj_smooth(&model, material, transform))
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
