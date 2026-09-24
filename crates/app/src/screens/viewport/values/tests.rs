//! What app · viewport/values.rs is held to.

use cao_part::{VariableChange, Variables};

use super::*;

fn half_of_a_width() -> Formula {
    let mut variables = Variables::default();
    variables.change(&VariableChange::Added {
        name: "largeur".to_string(),
        formula: Formula::Number(120.0),
    });
    variables.read("largeur / 2").expect("it reads")
}

#[test]
fn a_value_read_back_as_the_formula_typed_for_it_keeps_the_formula() {
    let half = half_of_a_width();

    assert_eq!(as_typed(Some((half.clone(), 60.0)), 60.000_000_001), half);
}

#[test]
fn a_value_the_drawing_reads_otherwise_is_the_number_it_reads() {
    let half = half_of_a_width();

    assert_eq!(as_typed(Some((half, 60.0)), 120.0), Formula::Number(120.0));
    assert_eq!(as_typed(None, 42.0), Formula::Number(42.0));
}

#[test]
fn a_plain_number_typed_is_left_as_the_drawing_reads_it() {
    assert_eq!(
        as_typed(Some((Formula::Number(40.0), 40.0)), 40.000_000_001),
        Formula::Number(40.000_000_001),
    );
}
