//! Decision 2 between two perpendicular cylinders: a near-touch is read
//! from the walls' own measure, whatever the curve `relation` builds on
//! them as they stand.
//!
//! What must not break, decision 2's sight: a wall a hair from a node with
//! another, which as they stand cross along two loops with no node, is
//! still not moved onto a third surface's touch that would leave the node
//! further off —
//! `a_wall_a_hair_from_a_node_is_not_moved_onto_a_plane_that_would_leave_the_node_further_off`.
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

#[test]
fn a_wall_a_hair_from_a_node_is_not_moved_onto_a_plane_that_would_leave_the_node_further_off() {
    let hair = scale().eps() / 4.0;
    let shaft = Cylinder::about(DVec3::ZERO, DVec3::Z, 10.0);
    let floor = plane_at(-2.0 * hair, DVec3::X);
    let bar = Cylinder::about(DVec3::new(5.0 - hair, 0.0, 10.0), DVec3::Y, 5.0);
    let Relation::Meet(meeting) =
        relation(&Surface::Cylinder(shaft), &Surface::Cylinder(bar), scale())
    else {
        panic!("the shaft and the bar meet");
    };
    assert!(
        meeting.nodes.is_empty() && meeting.contact.is_none(),
        "as they stand, the bar crosses the shaft with no node: {meeting:?}"
    );
    let mut surfaces = Surfaces {
        list: vec![Surface::Cylinder(shaft), floor, Surface::Cylinder(bar)],
        mapped: [Vec::new(), Vec::new()],
    };
    let moved = surfaces.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        |_, _, _| true,
        |_| false,
        scale(),
    );
    assert_eq!(
        surfaces.list,
        [Surface::Cylinder(shaft), floor, Surface::Cylinder(bar)],
        "the bar was moved off the node it stands a hair from: {moved:?}"
    );
}
