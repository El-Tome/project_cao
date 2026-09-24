//! What app · wording/formula.rs is held to.

use cao_part::VariableChange;

use super::*;

fn a_table() -> Variables {
    let mut variables = Variables::default();
    variables.change(&VariableChange::Added {
        name: "largeur".to_string(),
        formula: Formula::Number(120.0),
    });
    variables
}

fn rounded(value: f64) -> String {
    format!("{value:.0}")
}

#[test]
fn a_plain_number_is_said_as_the_number_it_is() {
    let lang = Catalogue::french();

    assert_eq!(
        sized(&lang, &a_table(), &Formula::Number(12.0), rounded),
        "12"
    );
}

#[test]
fn a_size_written_from_a_variable_says_what_it_was_written_as_and_what_it_comes_to() {
    let lang = Catalogue::french();
    let variables = a_table();
    let half = variables.read("largeur / 2").expect("it reads");

    assert_eq!(sized(&lang, &variables, &half, rounded), "largeur / 2 = 60");
}

#[test]
fn a_formula_that_does_not_read_is_told_what_is_wrong_with_it() {
    let lang = Catalogue::french();

    let unknown = unreadable(&lang, &Unreadable::UnknownName("hauteur".to_string()));
    let stray = unreadable(&lang, &Unreadable::StrayCharacter('$'));
    let missing = unreadable(&lang, &Unreadable::MissingValue(Some('*')));

    assert!(unknown.contains("hauteur"), "{unknown}");
    assert!(stray.contains('$'), "{stray}");
    assert!(missing.contains('*'), "{missing}");
    for wrong in [
        Unreadable::Empty,
        Unreadable::MissingValue(None),
        Unreadable::MissingOperator,
        Unreadable::Unclosed,
        Unreadable::Unopened,
    ] {
        let said = unreadable(&lang, &wrong);
        assert!(
            !said.starts_with("formula."),
            "{wrong:?} has no sentence of its own: {said}"
        );
    }
}
