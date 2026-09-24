//! What a formula reads, what it comes to, and how it is written back.
//!
//! Closes #175.
//! - a formula reads numbers, the four operations, parentheses and other
//!   variables' names —
//!   `the_four_operations_keep_their_usual_order_and_parentheses_change_it`,
//!   `a_name_reads_as_the_variable_it_names_and_comes_to_its_value`
//! - one that does not read says what is wrong with it —
//!   `a_formula_that_does_not_read_says_what_is_wrong_with_it`,
//!   `a_name_the_part_has_no_variable_for_is_refused_by_name`
//! - it goes into the part file by rank, so a name changed later changes
//!   nothing written — `a_formula_goes_into_a_file_by_rank_and_comes_back_the_same`

use super::*;

const TOLERANCE: f64 = 1e-9;

fn no_names(_: &str) -> Option<VariableId> {
    None
}

fn comes_to(text: &str) -> f64 {
    Formula::read(text, no_names)
        .expect("a formula that reads")
        .value(&[])
        .expect("a formula that comes to a number")
}

#[test]
fn a_plain_number_reads_as_that_number() {
    assert!((comes_to("60") - 60.0).abs() < TOLERANCE);
    assert!((comes_to(" 12.5 ") - 12.5).abs() < TOLERANCE);
}

#[test]
fn the_four_operations_keep_their_usual_order_and_parentheses_change_it() {
    for (text, expected) in [
        ("2 + 3 * 4", 14.0),
        ("(2 + 3) * 4", 20.0),
        ("10 - 4 - 3", 3.0),
        ("8 / 4 / 2", 1.0),
        ("-3 + 5", 2.0),
        ("2 * -3", -6.0),
        ("-(1 + 2) * 2", -6.0),
        ("((7))", 7.0),
    ] {
        let got = comes_to(text);
        assert!(
            (got - expected).abs() < TOLERANCE,
            "{text} came to {got}, not {expected}",
        );
    }
}

#[test]
fn a_formula_may_open_on_an_equals_sign_and_use_a_decimal_comma() {
    assert!((comes_to("=7") - 7.0).abs() < TOLERANCE);
    assert!((comes_to("2,5 * 2") - 5.0).abs() < TOLERANCE);
    assert!((comes_to(".5") - 0.5).abs() < TOLERANCE);
}

fn plate(name: &str) -> Option<VariableId> {
    match name {
        "width" => Some(VariableId(0)),
        "α_2" => Some(VariableId(1)),
        _ => None,
    }
}

#[test]
fn a_name_reads_as_the_variable_it_names_and_comes_to_its_value() {
    let formula = Formula::read("width / 2 + α_2", plate).expect("it reads");

    let value = formula.value(&[100.0, 3.0]).expect("it comes to a number");

    assert!((value - 53.0).abs() < TOLERANCE, "came to {value}");
}

#[test]
fn a_name_the_part_has_no_variable_for_is_refused_by_name() {
    assert_eq!(
        Formula::read("width * height", plate),
        Err(Unreadable::UnknownName("height".to_string())),
    );
}

#[test]
fn a_formula_that_does_not_read_says_what_is_wrong_with_it() {
    for (text, wrong) in [
        ("", Unreadable::Empty),
        ("  = ", Unreadable::Empty),
        ("width $ 2", Unreadable::StrayCharacter('$')),
        ("==5", Unreadable::StrayCharacter('=')),
        ("width +", Unreadable::MissingValue(None)),
        ("* 2", Unreadable::MissingValue(Some('*'))),
        ("()", Unreadable::MissingValue(Some(')'))),
        ("2 width", Unreadable::MissingOperator),
        ("(2)(3)", Unreadable::MissingOperator),
        ("(width + 2", Unreadable::Unclosed),
        ("width + 2)", Unreadable::Unopened),
    ] {
        assert_eq!(Formula::read(text, plate), Err(wrong), "reading {text:?}");
    }
}

#[test]
fn a_division_by_zero_comes_to_no_number_at_all() {
    let formula = Formula::read("width / (α_2 - 3)", plate).expect("it reads");

    assert_eq!(formula.value(&[100.0, 3.0]), None);
}

#[test]
fn a_variable_that_comes_to_nothing_leaves_the_formula_at_nothing() {
    let formula = Formula::read("width + 1", plate).expect("it reads");

    assert_eq!(formula.value(&[f64::NAN]), None);
    assert_eq!(formula.value(&[]), None);
}

fn plate_name(variable: VariableId) -> String {
    ["width", "α_2"][variable.0].to_string()
}

#[test]
fn a_formula_is_written_back_with_the_names_and_only_the_parentheses_it_needs() {
    for (typed, written) in [
        ("width/2+5", "width / 2 + 5"),
        ("((width + α_2)) * 2", "(width + α_2) * 2"),
        ("width - (α_2 - 1)", "width - (α_2 - 1)"),
        ("width - α_2 - 1", "width - α_2 - 1"),
        ("-(width + 1)", "-(width + 1)"),
        ("2 * -width", "2 * -width"),
        ("= 60", "60"),
        ("2,5", "2.5"),
        ("-30", "-30"),
    ] {
        let formula = Formula::read(typed, plate).expect("it reads");

        assert_eq!(formula.written(plate_name), written, "writing {typed:?}");
    }
}

#[test]
fn a_formula_written_back_reads_as_the_same_formula() {
    for typed in [
        "width / 2 + 5",
        "width - (α_2 - (1 - width))",
        "width / (α_2 / 2)",
        "-(-width) * 0.125",
        "1 + 2 + 3 * 4 / 5 - -6",
    ] {
        let formula = Formula::read(typed, plate).expect("it reads");

        let again = Formula::read(&formula.written(plate_name), plate);

        assert_eq!(again, Ok(formula), "reading back {typed:?}");
    }
}

#[test]
fn a_formula_goes_into_a_file_by_rank_and_comes_back_the_same() {
    let formula = Formula::read("width / 2 + α_2", plate).expect("it reads");

    let text = serde_json::to_string(&formula).expect("it goes into a file");
    let back: Formula = serde_json::from_str(&text).expect("it comes back out");

    assert_eq!(text, "\"#0 / 2 + #1\"");
    assert_eq!(back, formula);
}

#[test]
fn a_plain_number_goes_into_a_file_as_a_number_and_a_number_reads_as_one() {
    let text = serde_json::to_string(&Formula::Number(60.0)).expect("it goes into a file");
    let back: Formula = serde_json::from_str("12.5").expect("a number reads");

    assert_eq!(text, "60.0");
    assert_eq!(back, Formula::Number(12.5));
}

#[test]
fn a_count_comes_out_whole_or_not_at_all() {
    let count =
        |text: &str, values: &[f64]| Formula::read(text, plate).expect("it reads").whole(values);

    assert_eq!(count("4", &[]), Some(4));
    assert_eq!(count("width / 10", &[120.0]), Some(12));
    assert_eq!(
        count("width / 3 * 3", &[10.0]),
        Some(10),
        "a hair off whole is whole"
    );
    assert_eq!(count("2.5", &[]), None);
    assert_eq!(count("-3", &[]), None);
}

#[test]
fn a_size_taken_twice_stays_a_plain_number_when_it_was_one() {
    let half = Formula::read("width / 2", plate).expect("it reads");

    assert_eq!(Formula::Number(30.0).times(2.0), Formula::Number(60.0));
    assert_eq!(half.clone().times(2.0).written(plate_name), "width / 2 * 2");
    assert_eq!(half.times(2.0).value(&[100.0]), Some(100.0));
}
