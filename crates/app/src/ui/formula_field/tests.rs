//! What app · ui/formula_field.rs is held to: which name the cursor is in, and
//! which of the names offered go on from what was typed.

use super::*;

const NAMING: Naming = Naming {
    starts: |character| character.is_alphabetic() || character == '_',
    goes_on: |character| character.is_alphanumeric() || character == '_',
};

fn offers(names: &[&str]) -> Vec<Offer> {
    names
        .iter()
        .map(|name| Offer {
            name: (*name).to_string(),
            beside: String::new(),
        })
        .collect()
}

fn named<'a>(offered: &[&'a Offer]) -> Vec<&'a str> {
    offered.iter().map(|offer| offer.name.as_str()).collect()
}

#[test]
fn the_name_being_typed_is_the_run_of_name_characters_the_cursor_stands_in() {
    assert_eq!(name_at("wid", 3, NAMING), Some((0..3, "wid".to_string())));
    assert_eq!(
        name_at("2 * wid", 7, NAMING),
        Some((4..7, "wid".to_string()))
    );
    assert_eq!(name_at("(wid", 4, NAMING), Some((1..4, "wid".to_string())));
    assert_eq!(name_at("a2", 2, NAMING), Some((0..2, "a2".to_string())));
    assert_eq!(
        name_at("widTH/2", 3, NAMING),
        Some((0..5, "wid".to_string())),
        "the whole name is what a choice replaces, what stands before the \
         cursor is what the list goes by"
    );
    assert_eq!(
        name_at("épais", 5, NAMING),
        Some((0..5, "épais".to_string()))
    );
}

#[test]
fn nothing_is_being_named_after_a_number_an_operator_or_nothing() {
    assert_eq!(name_at("12", 2, NAMING), None);
    assert_eq!(name_at("3a", 2, NAMING), None);
    assert_eq!(name_at("2 * ", 4, NAMING), None);
    assert_eq!(name_at("", 0, NAMING), None);
    assert_eq!(name_at("width", 0, NAMING), None, "the cursor before it");
}

#[test]
fn the_names_offered_are_those_that_go_on_from_what_was_typed_case_aside() {
    let offered = offers(&["width", "Height", "wall", "épaisseur"]);

    assert_eq!(named(&starting_with(&offered, "w")), vec!["width", "wall"]);
    assert_eq!(named(&starting_with(&offered, "HE")), vec!["Height"]);
    assert_eq!(named(&starting_with(&offered, "É")), vec!["épaisseur"]);
    assert!(starting_with(&offered, "x").is_empty());
}

#[test]
fn a_name_chosen_takes_the_place_of_the_whole_name_the_cursor_was_in() {
    assert_eq!(
        replaced("2*wi", &(2..4), "width"),
        ("2*width".to_string(), 7)
    );
    assert_eq!(
        replaced("wiDTH+1", &(0..5), "width"),
        ("width+1".to_string(), 5)
    );
    assert_eq!(
        replaced("é+é", &(2..3), "épaisseur"),
        ("é+épaisseur".to_string(), 11)
    );
}
