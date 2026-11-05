//! What app · wording/copy_panel.rs is held to.
//!
//! Closes #321.
//! - each row of the panel is said by the language file, one entry per row and
//!   state, French spacing and agreement included — `the_elements_row_counts_what_is_held`,
//!   `the_target_row_says_whether_it_is_still_to_click`,
//!   `a_direction_chosen_agrees_with_its_noun`

use super::*;

#[test]
fn the_elements_row_counts_what_is_held() {
    assert_eq!(elements(&Catalogue::french(), 3), "Éléments : 3");
}

#[test]
fn the_target_row_says_whether_it_is_still_to_click() {
    let lang = Catalogue::french();

    assert_eq!(
        target(&lang, Target::Axis, false),
        "Axe : à cliquer dans le dessin"
    );
    assert_eq!(target(&lang, Target::Axis, true), "Axe : choisi");
    assert_eq!(target(&lang, Target::Centre, true), "Centre : choisi");
    assert_eq!(
        target(&lang, Target::Direction, false),
        "Direction : à cliquer dans le dessin"
    );
}

#[test]
fn a_direction_chosen_agrees_with_its_noun() {
    assert_eq!(
        target(&Catalogue::french(), Target::Direction, true),
        "Direction : choisie"
    );
}
