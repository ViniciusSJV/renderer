// @TODO: Find an implementation way to prevent the "field does not implement `Copy`" error for type Vec using a bidirectional tree structure

use std::ops::Index;
use crate::matrix::Matrix;
use crate::object::{Intersectable, Object};
use crate::bounds::Bounds;
use crate::intersection::Intersections;
use crate::ray::Ray;

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {
    pub children: Vec<Object>,
    pub subgroups: Vec<Groups>,
    pub transform: Matrix<4>,
    pub local_bounds: Bounds,
}

impl Default for Groups {
    fn default() -> Self {
        Groups::from(vec![])
    }
}

impl From<Vec<Object>> for Groups {
    fn from(children: Vec<Object>) -> Self {
        Self::new(children)
    }
}

impl Index<usize> for Groups {
    type Output = Object;
    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}

impl IntoIterator for Groups {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl Groups {
    fn new(children: Vec<Object>) -> Self {
        let mut local_bounds = Bounds::empty();
        for child in &children {
            local_bounds = local_bounds.union(child.bounds());
        }
        Groups {
            children,
            subgroups: vec![],
            transform: Matrix::identity(),
            local_bounds,
        }
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn into_objects(self) -> Vec<Object> {
        self.children
            .into_iter()
            .map(|mut child| {
                child.set_transform(self.transform * child.transform());
                child
            })
            .collect()
    }

    pub fn from_subgroups(subgroups: Vec<Groups>) -> Self {
        let mut local_bounds = Bounds::empty();
        for subgroup in &subgroups {
            local_bounds = local_bounds.union(subgroup.bounds());
        }
        Self { children: vec![], subgroups, transform: Matrix::identity(), local_bounds }
    }

    pub fn bounds(&self) -> Bounds {
        self.local_bounds.transformed(self.transform)
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let local_ray = ray.set_transform(self.transform.inverse());
        if !self.local_bounds.intersects(local_ray) {
            return Intersections::new(vec![]);
        }

        let mut intersections = vec![];
        for child in &self.children {
            intersections.extend(child.intersect(local_ray));
        }
        for subgroup in &self.subgroups {
            intersections.extend(subgroup.intersect(local_ray));
        }
        Intersections::new(intersections)
    }

}

#[cfg(test)]
mod tests_groups {
    use crate::groups::Groups;
    use crate::matrix::Matrix;

    #[test]
    fn creating_a_new_group() {
        let g = Groups::default();

        assert_eq!(g.transform, Matrix::identity());
        assert_eq!(g.len(), 0);
    }
}