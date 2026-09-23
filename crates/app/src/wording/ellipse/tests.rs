//! What app · wording/ellipse.rs is held to.

use super::*;

#[test]
fn each_way_of_drawing_an_ellipse_asks_for_something_different() {
    let lang = Catalogue::french();

    assert_ne!(
        asks_for(&lang, EllipseMode::ByCentre),
        asks_for(&lang, EllipseMode::ByEnds),
        "two modes asking for the same thing leave the user guessing which they are in",
    );
}
