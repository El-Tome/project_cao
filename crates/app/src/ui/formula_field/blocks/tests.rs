//! What app · ui/formula_field/blocks.rs is held to: which names are blocks,
//! and what the keys and a click do around them.

use super::*;

const NAMING: Naming = Naming {
    starts: |character| character.is_alphabetic() || character == '_',
    goes_on: |character| character.is_alphanumeric() || character == '_',
};

const NAMES: [&str; 3] = ["width", "wall", "épaisseur"];

/// Where `width` stands in `2 * width`.
const WIDTH: Range<usize> = 4..9;

fn blocks_in(text: &str) -> Vec<Range<usize>> {
    found(text, &NAMES, NAMING)
}

#[test]
fn every_whole_name_of_a_variable_is_a_block() {
    assert_eq!(blocks_in("2 * width"), vec![WIDTH]);
    assert_eq!(blocks_in("width+wall"), vec![0..5, 6..10]);
    assert_eq!(blocks_in("(épaisseur)"), vec![1..10]);
}

#[test]
fn a_name_not_whole_or_not_a_variable_is_plain_text() {
    assert!(blocks_in("wid").is_empty(), "still being typed");
    assert!(blocks_in("widths").is_empty(), "no variable goes by it");
    assert!(
        blocks_in("Width").is_empty(),
        "names are read as they are cased"
    );
    assert!(
        blocks_in("2width").is_empty(),
        "a run a digit opens is no name"
    );
    assert!(blocks_in("12.5 * 2").is_empty());
}

#[test]
fn a_step_crosses_a_block_whole_and_leaves_anything_else_to_the_field() {
    let blocks = [WIDTH];
    assert_eq!(stepped(9, false, &blocks), Some(4), "left from its end");
    assert_eq!(stepped(4, true, &blocks), Some(9), "right from its start");
    assert_eq!(stepped(3, false, &blocks), None);
    assert_eq!(stepped(9, true, &blocks), None);
}

#[test]
fn erasing_next_to_a_block_takes_it_whole() {
    let blocks = [WIDTH];
    assert_eq!(
        erased(9, false, &blocks),
        Some(WIDTH),
        "Retour arrière after it"
    );
    assert_eq!(erased(4, true, &blocks), Some(WIDTH), "Suppr before it");
    assert_eq!(erased(4, false, &blocks), None);
    assert_eq!(erased(8, false, &blocks), None);
}

#[test]
fn a_cursor_inside_a_block_goes_to_its_nearer_edge() {
    let blocks = [WIDTH];
    assert_eq!(snapped(5, &blocks), 4);
    assert_eq!(snapped(8, &blocks), 9);
    assert_eq!(snapped(4, &blocks), 4, "an edge is no inside");
    assert_eq!(snapped(2, &blocks), 2);
}

#[test]
fn a_block_is_laid_out_on_a_tint_and_the_rest_as_it_was() {
    let tint = egui::Color32::from_rgb(1, 2, 3);
    let job = laid_out(
        "2 * width + 1",
        &[WIDTH],
        egui::FontId::default(),
        egui::Color32::WHITE,
        tint,
    );

    let tinted: Vec<&str> = job
        .sections
        .iter()
        .filter(|section| section.format.background == tint)
        .map(|section| &job.text[section.byte_range.start.0..section.byte_range.end.0])
        .collect();
    assert_eq!(tinted, vec!["width"]);
    assert_eq!(job.text, "2 * width + 1");
}
