use crate::EPSILON;

pub trait Equivalence<T: Clone> {
    fn equivalent(&self, other: T) -> bool;

    fn not_equivalent(&self, other: T) -> bool {
        !self.equivalent(other)
    }
}

impl Equivalence<f64> for f64 {
    fn equivalent(&self, other: f64) -> bool {
        (*self - other).abs() < EPSILON
    }
}

#[macro_export]
macro_rules! assert_equivalent {
  ($left:expr, $right:expr $(,)?) => {{
    match (&$left, $right) {
      (left_val, right_val) => {
        if left_val.not_equivalent(right_val) {
          panic!(
            "asserting equality. {:?} is not equivalent to {:?}",
            left_val, right_val
          );
        }
      }
    }
  }};
}

#[macro_export]
macro_rules! not_equivalent {
  ($left:expr, $right:expr $(,)?) => {{
    match (&$left, $right) {
      (left_val, right_val) => {
        if left_val.equivalent(right_val) {
          panic!(
            "asserting in-equality. {:?} is equivalent equal to {:?}",
            left_val, right_val
          );
        }
      }
    }
  }};
}