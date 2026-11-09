//! What prefs · toolbar.rs is held to.

use super::*;

fn layout() -> ToolbarLayout {
    ToolbarLayout {
        items: vec![
            Item::Command(Command::Undo),
            Item::group("G", vec![Item::Command(Command::Redo)]),
            Item::Command(Command::NewSketch),
        ],
        ..ToolbarLayout::default()
    }
}

#[test]
fn an_entry_is_found_by_its_path() {
    let layout = layout();
    assert_eq!(layout.at(&[0]), Some(&Item::Command(Command::Undo)));
    assert_eq!(layout.at(&[1, 0]), Some(&Item::Command(Command::Redo)));
    assert_eq!(layout.at(&[9]), None);
    assert_eq!(layout.at(&[0, 0]), None, "a command has no children");
}

#[test]
fn shifting_swaps_with_the_neighbour() {
    let mut layout = layout();
    assert_eq!(layout.shift(&[0], true), Some(vec![1]));
    assert_eq!(layout.at(&[1]), Some(&Item::Command(Command::Undo)));
    assert_eq!(layout.shift(&[0], false), None, "nothing before the first");
}

/// Groups within groups within groups : nesting has no floor.
#[test]
fn nesting_goes_as_deep_as_asked() {
    let mut layout = ToolbarLayout {
        items: vec![
            Item::group("A", vec![]),
            Item::group("B", vec![]),
            Item::Command(Command::Undo),
        ],
        ..ToolbarLayout::default()
    };

    assert_eq!(layout.nest(&[2]), Some(vec![1, 0]), "into B");
    assert_eq!(layout.nest(&[1]), Some(vec![0, 0]), "B into A");
    assert_eq!(layout.at(&[0, 0, 0]), Some(&Item::Command(Command::Undo)));
}

#[test]
fn an_entry_comes_back_out_next_to_its_group() {
    let mut layout = layout();
    assert_eq!(layout.unnest(&[1, 0]), Some(vec![2]));
    assert_eq!(layout.at(&[2]), Some(&Item::Command(Command::Redo)));
    assert_eq!(layout.at(&[3]), Some(&Item::Command(Command::NewSketch)));
    assert_eq!(layout.unnest(&[0]), None, "already at the root");
}

#[test]
fn nesting_needs_a_group_to_go_into() {
    let mut layout = layout();
    assert_eq!(layout.nest(&[1]), None, "what comes before it is a command");
}

#[test]
fn an_entry_can_be_taken_out_and_put_back() {
    let mut layout = layout();
    let taken = layout
        .remove(&[1, 0])
        .expect("the command inside the group");
    assert_eq!(taken, Item::Command(Command::Redo));
    assert!(layout.push_into(&[1], taken));
    assert_eq!(layout.at(&[1, 0]), Some(&Item::Command(Command::Redo)));
}

#[test]
fn only_a_group_can_be_renamed() {
    let mut layout = layout();
    assert!(layout.rename(&[1], "Other".to_string()));
    assert!(matches!(layout.at(&[1]), Some(Item::Group { name, .. }) if name == "Other"));
    assert!(!layout.rename(&[0], "Nothing".to_string()));
}
