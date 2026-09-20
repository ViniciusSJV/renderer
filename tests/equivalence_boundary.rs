use renderer::{assert_equivalent, equivalent::Equivalence, EPSILON};

#[test]
fn accepts_equal_values() {
    assert_equivalent!(0.0_f64, 0.0_f64);
}

#[test]
fn accepts_difference_below_epsilon() {
    assert_equivalent!(0.0_f64, EPSILON / 2.0);
}

#[test]
#[should_panic(expected = "asserting equality.")]
fn rejects_difference_at_epsilon() {
    assert_equivalent!(0.0_f64, EPSILON);
}

#[test]
#[should_panic(expected = "asserting equality.")]
fn rejects_difference_above_epsilon() {
    assert_equivalent!(0.0_f64, EPSILON * 2.0);
}
