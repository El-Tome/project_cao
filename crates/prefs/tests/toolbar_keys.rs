//! A group of the standard layout is matched by name when a new command has to
//! find its place, so its name is an identity, not a sentence. The interface
//! says how each one reads.
//!
//! An integration test rather than a module inside `toolbar.rs`: that file is
//! past the line budget, so the gate refuses it any growth at all.

use cao_prefs::{Item, ToolbarLayout};

fn keys_of(items: &[Item], out: &mut Vec<String>) {
    for item in items {
        if let Item::Group { name, items } = item {
            out.push(name.clone());
            keys_of(items, out);
        }
    }
}

#[test]
fn every_group_of_the_standard_toolbar_is_named_by_a_key() {
    let mut keys = Vec::new();
    keys_of(&ToolbarLayout::default().items, &mut keys);

    assert!(!keys.is_empty(), "the standard toolbar holds groups");
    for key in keys {
        assert!(
            key.chars()
                .all(|letter| letter.is_ascii_lowercase() || letter == '_'),
            "{key:?} is not a key: a name the interface says in its own words \
             holds no accent and no capital",
        );
    }
}
