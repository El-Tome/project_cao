//! Decision 2 between two perpendicular cylinders: a near-touch is read
//! from the walls' own measure, whatever the curve `relation` builds on
//! them as they stand.
//!
//! What must not break, decision 2's sight: a wall a hair from a node with
//! another, which as they stand cross along two loops with no node, is
//! still not moved onto a third surface's touch that would leave the node
//! further off —
//! `a_wall_a_hair_from_a_node_is_not_moved_onto_a_plane_that_would_leave_the_node_further_off`.
//! An exact node holds a wall the same way —
//! `a_wall_exactly_on_a_node_is_not_moved_onto_a_plane_that_would_open_the_node` —
//! and a crossing further than the tolerance from one holds nothing —
//! `a_wall_further_than_the_tolerance_from_a_node_is_moved_onto_a_plane_though_it_lands_a_hair_from_the_node`.
//! A wall its operand drew corners on, exactly on a node, is read on the
//! node `Meeting::of` decides: moved off it by less than a thousandth of the
//! tolerance, the node is still there to measure —
//! `a_wall_with_corners_exactly_on_a_node_is_moved_off_it_by_less_than_a_thousandth_of_the_tolerance` —
//! and slid along it by a hair, the move is refused —
//! `a_wall_with_corners_exactly_on_a_node_is_not_slid_along_it_by_a_hair`.
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

/// A shaft of radius 10 about Z, a floor of the first operand, and a bar of
/// radius 5 along Y through `(x, z)`, the second operand's, its corners on
/// it where `cornered` says so, as decision 2 leaves them: the bar, and
/// whether it was moved.
fn bar_beside_a_shaft(floor: Surface, [x, z]: [f64; 2], cornered: bool) -> (Cylinder, bool) {
    let shaft = Cylinder::about(DVec3::ZERO, DVec3::Z, 10.0);
    let bar = Cylinder::about(DVec3::new(x, 0.0, z), DVec3::Y, 5.0);
    let mut surfaces = Surfaces {
        list: vec![Surface::Cylinder(shaft), floor, Surface::Cylinder(bar)],
        mapped: [Vec::new(), Vec::new()],
    };
    let moved = surfaces.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        |_, _, _| true,
        |surface| cornered && surface.0 == 2,
        scale(),
    );
    assert_eq!(surfaces.list[..2], [Surface::Cylinder(shaft), floor]);
    let Surface::Cylinder(bar) = surfaces.list[2] else {
        panic!("the bar stays a cylinder");
    };
    (bar, moved[2])
}

#[test]
fn a_wall_exactly_on_a_node_is_not_moved_onto_a_plane_that_would_open_the_node() {
    let hair = scale().eps() / 4.0;
    let (bar, moved) = bar_beside_a_shaft(plane_at(-hair, DVec3::X), [5.0, 10.0], false);
    assert!(!moved, "the bar was moved off its node, to {bar:?}");
}

#[test]
fn a_wall_further_than_the_tolerance_from_a_node_is_moved_onto_a_plane_though_it_lands_a_hair_from_the_node()
 {
    let eps = scale().eps();
    let floor = -0.5 * eps;
    let (bar, moved) =
        bar_beside_a_shaft(plane_at(floor, DVec3::X), [5.0 - 1.25 * eps, 10.0], false);
    assert!(
        moved,
        "a crossing that is no touch held the bar where it was"
    );
    assert!(
        (bar.origin.x - bar.radius - floor).abs() < 1e-14,
        "the bar stands {:e} off the floor",
        bar.origin.x - bar.radius - floor
    );
}

#[test]
fn a_wall_with_corners_exactly_on_a_node_is_moved_off_it_by_less_than_a_thousandth_of_the_tolerance()
 {
    let off = scale().eps() / 2e3;
    let (bar, moved) = bar_beside_a_shaft(plane_at(-off, DVec3::X), [5.0, 10.0], true);
    assert!(moved, "the bar was held on its node");
    assert!(
        (bar.origin.x - bar.radius + off).abs() < 1e-14,
        "the bar stands {:e} off the floor",
        bar.origin.x - bar.radius + off
    );
}

#[test]
fn a_wall_with_corners_exactly_on_a_node_is_not_slid_along_it_by_a_hair() {
    let hair = scale().eps() / 4.0;
    let (bar, moved) = bar_beside_a_shaft(plane_at(-(15.0 + hair), -DVec3::Z), [5.0, 10.0], true);
    assert!(!moved, "the bar's node was slid along, to {bar:?}");
}
