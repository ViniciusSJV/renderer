// @TODO: Find an implementation way to prevent the "field does not implement `Copy`" error for type Vec

use std::ops::Index;
use crate::matrix::Matrix;
use crate::object::Object;

// GROUPS

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {
    data: Vec<Object>,
    pub transform: Matrix<4>
}

impl Default for Groups {
    fn default() -> Self {
        Groups::from(vec![])
    }
}

impl From<Vec<Object>> for Groups {
    fn from(v: Vec<Object>) -> Self {
        Self::new(v)
    }
}

impl Index<usize> for Groups {
    type Output = Object;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IntoIterator for Groups {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Groups {
    fn new(data: Vec<Object>) -> Self {
        Groups {
            data,
            transform: Matrix::identity()
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// GROUPS

// PARENT

#[derive(Debug, PartialEq, Clone)]
pub struct Parent {
    data: Vec<Object>
}

impl Default for Parent {
    fn default() -> Self {
        Parent::from(vec![])
    }
}

impl IntoIterator for Parent {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl From<Vec<Object>> for Parent {
    fn from(v: Vec<Object>) -> Self {
        Self::new(v)
    }
}

impl Index<usize> for Parent {
    type Output = Object;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Parent {
    fn new(data: Vec<Object>) -> Self {
        Parent {
            data
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// PARENT

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