//! What app · adapters/save_dialog.rs is held to.

use std::path::PathBuf;

use super::with_stl_extension;

#[test]
fn a_place_chosen_without_an_extension_is_given_stl() {
    assert_eq!(
        with_stl_extension(PathBuf::from("/exports/plaque")),
        PathBuf::from("/exports/plaque.stl"),
    );
}

#[test]
fn a_place_chosen_with_stl_keeps_it_in_whatever_case() {
    for chosen in ["/exports/plaque.stl", "/exports/plaque.STL"] {
        assert_eq!(
            with_stl_extension(PathBuf::from(chosen)),
            PathBuf::from(chosen)
        );
    }
}

#[test]
fn a_name_holding_a_dot_is_still_given_stl() {
    assert_eq!(
        with_stl_extension(PathBuf::from("/exports/plaque v1.2")),
        PathBuf::from("/exports/plaque v1.2.stl"),
    );
}
