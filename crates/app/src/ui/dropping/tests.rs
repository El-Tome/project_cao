//! What app · ui/dropping.rs is held to: where a name dropped into a field
//! goes.

use super::*;

#[test]
fn a_name_dropped_takes_the_place_of_the_text_selected() {
    assert_eq!(
        dropped_into("162.12", Some(0..6), 3, "width"),
        ("width".to_string(), 5)
    );
    assert_eq!(
        dropped_into("2 * 3", Some(4..5), 0, "wall"),
        ("2 * wall".to_string(), 8)
    );
}

#[test]
fn with_nothing_selected_a_plain_number_gives_way_to_it_whole() {
    assert_eq!(
        dropped_into("162.12", None, 4, "width"),
        ("width".to_string(), 5)
    );
    assert_eq!(
        dropped_into(" 40,5 ", None, 1, "width"),
        ("width".to_string(), 5)
    );
    assert_eq!(dropped_into("", None, 7, "width"), ("width".to_string(), 5));
}

#[test]
fn with_nothing_selected_it_goes_into_a_calculation_where_it_lands() {
    assert_eq!(
        dropped_into("2 * ", None, 4, "width"),
        ("2 * width".to_string(), 9)
    );
    assert_eq!(
        dropped_into("+ 1", None, 0, "width"),
        ("width+ 1".to_string(), 5)
    );
    assert_eq!(
        dropped_into("2 *", None, 9, "width"),
        ("2 *width".to_string(), 8),
        "never past the end"
    );
}
