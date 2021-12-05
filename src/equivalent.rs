const EPSILON: f64 = 0.00001;

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
            "asserting fuzzy equality. {:?} is not fuzzy equal to {:?}",
            left_val, right_val
          );
        }
      }
    }
  }};
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
  ($left:expr, $right:expr $(,)?) => {{
    match (&$left, $right) {
      (left_val, right_val) => {
        if left_val.equivalent(right_val) {
          panic!(
            "asserting fuzzy in-equality. {:?} is fuzzy equal to {:?}",
            left_val, right_val
          );
        }
      }
    }
  }};
}