//! What app · wording/circle.rs is held to.

use super::*;
use std::collections::BTreeSet;

#[test]
fn each_way_of_drawing_a_circle_asks_for_something_different() {
    let lang = Catalogue::french();
    let modes = [
        CircleMode::Center,
        CircleMode::TwoPoints,
        CircleMode::ThreePoints,
        CircleMode::TwoTangents,
        CircleMode::ThreeTangents,
    ];
    let said: BTreeSet<String> = modes.iter().map(|mode| asks_for(&lang, *mode)).collect();

    assert_eq!(
        said.len(),
        modes.len(),
        "two modes asking for the same thing leave the user guessing which they are in"
    );
}
