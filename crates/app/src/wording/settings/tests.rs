//! What app · wording/settings.rs is held to.
//!
//! Closes #519.
//! - the top of the measure's figures slider says it means every figure, not
//!   one more — `the_top_of_the_measure_figures_slider_says_it_means_all_of_them`
//! - the slider showing it — no test: `settings/viewport.rs` draws it, which
//!   needs an `egui::Ui`

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

#[test]
fn the_top_of_the_measure_figures_slider_says_it_means_all_of_them() {
    let lang = Catalogue::french();
    let top = cao_prefs::config::ALL_FIGURES;

    assert_eq!(
        measure_figures(&lang, cao_prefs::config::FEWEST_FIGURES),
        "3"
    );
    assert_eq!(measure_figures(&lang, top - 1), (top - 1).to_string());
    let all = measure_figures(&lang, top);
    assert!(
        all.to_lowercase().contains("tous") && !all.chars().any(|c| c.is_ascii_digit()),
        "the top of the slider gives every figure a tenth of a micron needs, eight \
         on a metre, and reads {all:?} — one more than the notch below it",
    );
}
