//! What the table of a part's variables is held to.

use super::*;

const TOLERANCE: f64 = 1e-9;

fn added(name: &str, formula: Formula) -> VariableChange {
    VariableChange::Added {
        name: name.to_string(),
        formula,
    }
}

fn table(changes: &[VariableChange]) -> Variables {
    let mut variables = Variables::default();
    for change in changes {
        variables.change(change);
    }
    variables
}

fn value_of(variables: &Variables, name: &str) -> f64 {
    let variable = variables.named(name).expect("a variable of that name");
    variables.values()[variable.0]
}

#[test]
fn a_variable_added_is_found_by_its_name_and_comes_to_its_value() {
    let variables = table(&[added("width", Formula::Number(120.0))]);

    assert_eq!(variables.named("width"), Some(VariableId(0)));
    assert!((value_of(&variables, "width") - 120.0).abs() < TOLERANCE);
}

fn written(variables: &Variables, text: &str) -> Formula {
    variables
        .read(text)
        .expect("a formula that reads against the table")
}

#[test]
fn a_variable_written_from_others_comes_to_what_they_come_to() {
    let mut variables = table(&[added("width", Formula::Number(120.0))]);
    let half = written(&variables, "width / 2 + 5");
    variables.change(&added("middle", half));

    assert!((value_of(&variables, "middle") - 65.0).abs() < TOLERANCE);
}

#[test]
fn a_variable_can_lean_on_one_made_after_it() {
    let mut variables = table(&[
        added("a", Formula::Number(1.0)),
        added("b", Formula::Number(4.0)),
    ]);
    let twice_b = written(&variables, "b * 2");
    variables.change(&VariableChange::Edited {
        variable: VariableId(0),
        name: "a".to_string(),
        formula: twice_b,
    });

    assert!((value_of(&variables, "a") - 8.0).abs() < TOLERANCE);
}

#[test]
fn renaming_a_variable_renames_it_in_every_formula_written_from_it() {
    let mut variables = table(&[added("width", Formula::Number(120.0))]);
    let half = written(&variables, "width / 2");
    variables.change(&VariableChange::Edited {
        variable: VariableId(0),
        name: "l".to_string(),
        formula: Formula::Number(120.0),
    });

    assert_eq!(variables.written(&half), "l / 2");
    assert_eq!(variables.named("width"), None);
}

#[test]
fn an_erased_variable_goes_by_no_name_and_what_leaned_on_it_keeps_its_value() {
    let mut variables = table(&[added("width", Formula::Number(120.0))]);
    let half = written(&variables, "width / 2");
    variables.change(&VariableChange::Erased {
        variable: VariableId(0),
    });

    assert_eq!(variables.named("width"), None);
    assert_eq!(variables.live().count(), 0);
    assert_eq!(half.value(&variables.values()), Some(60.0));
}

#[test]
fn a_name_given_again_after_an_erasing_is_a_new_variable() {
    let mut variables = table(&[added("width", Formula::Number(120.0))]);
    variables.change(&VariableChange::Erased {
        variable: VariableId(0),
    });
    variables.change(&added("width", Formula::Number(80.0)));

    assert_eq!(variables.named("width"), Some(VariableId(1)));
    assert!((value_of(&variables, "width") - 80.0).abs() < TOLERANCE);
}

#[test]
fn a_name_is_letters_digits_and_underscores_and_never_opens_on_a_digit() {
    let variables = table(&[added("width", Formula::Number(120.0))]);

    for good in ["height", "α_2", "_margin", "L"] {
        assert_eq!(variables.check_name(good, None), Ok(()), "{good:?}");
    }
    for (bad, problem) in [
        ("", NameProblem::Empty),
        ("2nd_hole", NameProblem::OpensOnADigit),
        ("half width", NameProblem::NotAllowed(' ')),
        ("a-b", NameProblem::NotAllowed('-')),
        ("width", NameProblem::Taken),
    ] {
        assert_eq!(variables.check_name(bad, None), Err(problem), "{bad:?}");
    }
}

#[test]
fn a_variable_keeps_its_own_name_when_its_row_is_edited() {
    let variables = table(&[added("width", Formula::Number(120.0))]);

    assert_eq!(variables.check_name("width", Some(VariableId(0))), Ok(()));
}

#[test]
fn the_name_an_erased_variable_went_by_is_free_again() {
    let mut variables = table(&[added("width", Formula::Number(120.0))]);
    variables.change(&VariableChange::Erased {
        variable: VariableId(0),
    });

    assert_eq!(variables.check_name("width", None), Ok(()));
}

#[test]
fn a_formula_that_would_lean_on_itself_names_the_loop_it_closes() {
    let mut variables = table(&[added("a", Formula::Number(1.0))]);
    let from_a = written(&variables, "a + 1");
    variables.change(&added("b", from_a));
    variables.change(&added("c", Formula::Number(3.0)));
    let from_b = written(&variables, "c + b * 2");

    assert_eq!(
        variables.loop_through(VariableId(0), &from_b),
        Some(vec![VariableId(0), VariableId(1)]),
    );
    assert_eq!(
        variables.loop_through(VariableId(0), &written(&variables, "a")),
        Some(vec![VariableId(0)]),
    );
    assert_eq!(
        variables.loop_through(VariableId(2), &written(&variables, "b * 2")),
        None,
        "c may lean on b, which leans on a: nothing comes back to c",
    );
}

#[test]
fn a_loop_written_into_a_file_by_another_hand_comes_to_nothing_rather_than_hanging() {
    let mut variables = table(&[
        added("a", Formula::Number(1.0)),
        added("b", Formula::Number(2.0)),
        added("c", Formula::Number(3.0)),
    ]);
    let from_b = written(&variables, "b + 1");
    let from_a = written(&variables, "a + 1");
    variables.change(&VariableChange::Edited {
        variable: VariableId(0),
        name: "a".to_string(),
        formula: from_b,
    });
    variables.change(&VariableChange::Edited {
        variable: VariableId(1),
        name: "b".to_string(),
        formula: from_a,
    });

    let values = variables.values();

    assert!(values[0].is_nan() && values[1].is_nan(), "{values:?}");
    assert!((values[2] - 3.0).abs() < TOLERANCE);
}

#[test]
fn a_size_typed_as_text_is_read_and_worked_out_or_said_unusable() {
    let variables = table(&[added("width", Formula::Number(120.0))]);

    let (formula, value) = variables.size_of("=width / 4").expect("it reads");
    assert!((value - 30.0).abs() < TOLERANCE);
    assert_eq!(variables.written(&formula), "width / 4");
    assert_eq!(
        variables.size_of("width +"),
        Err(Unusable::Unreadable(Unreadable::MissingValue(None)))
    );
    assert_eq!(variables.size_of("width / 0"), Err(Unusable::NoNumber));
}
