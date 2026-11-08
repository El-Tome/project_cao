//! Decision 2 between two perpendicular cylinders: a near-touch is read
//! from the walls' own measure, whatever the curve `relation` builds on
//! them as they stand.
//!
//! Closes #776.
//! - Two perpendicular cylinders half the tolerance apart outside are still
//!   moved onto their contact by decision 2 —
//!   `two_perpendicular_cylinders_half_the_tolerance_apart_outside_are_moved_onto_their_contact`

use super::*;

#[test]
fn two_perpendicular_cylinders_half_the_tolerance_apart_outside_are_moved_onto_their_contact() {
    let shaft = Cylinder::about(DVec3::ZERO, DVec3::Z, 10.0);
    let bar = Cylinder::about(
        DVec3::new(0.0, 15.0 + scale().eps() / 2.0, 5.0),
        DVec3::X,
        5.0,
    );
    let [kept, moved] = snapped_pair(Surface::Cylinder(shaft), Surface::Cylinder(bar), false)[..]
    else {
        panic!("two surfaces");
    };
    assert_eq!(kept, Surface::Cylinder(shaft));
    let Surface::Cylinder(moved) = moved else {
        panic!("a cylinder");
    };
    assert_eq!(moved.radius, bar.radius);
    assert!(
        (moved.origin.y - moved.radius - shaft.radius).abs() < 1e-14,
        "the bar stands {:e} off its contact with the shaft",
        moved.origin.y - moved.radius - shaft.radius
    );
}
