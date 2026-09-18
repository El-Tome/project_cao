//! What app · wording/arc.rs is held to.

use super::*;

#[test]
fn each_way_of_drawing_an_arc_asks_for_something_different() {
    let lang = Catalogue::french();

    assert_ne!(
        asks_for(&lang, ArcMode::ByCenter),
        asks_for(&lang, ArcMode::ByEnds),
        "two modes asking for the same thing leave the user guessing which they are in",
    );
}
