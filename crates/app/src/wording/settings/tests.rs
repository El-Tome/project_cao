//! What app · wording/settings.rs is held to.

use super::*;

#[test]
fn the_profile_that_always_exists_reads_as_a_name_and_not_as_its_key() {
    assert_ne!(
        profile(&Catalogue::french(), DEFAULT_PROFILE),
        DEFAULT_PROFILE,
        "the settings screen offers the key it is stored under",
    );
}

/// A profile the user made carries their own words: the interface has
/// nothing to say about it.
#[test]
fn a_profile_the_user_named_reads_as_they_named_it() {
    assert_eq!(profile(&Catalogue::french(), "Atelier"), "Atelier");
}
